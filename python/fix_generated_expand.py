"""Patch a burn-onnx generated model so zero-length `Expand` targets work.

burn-onnx 0.22.0 lowers ONNX `Expand` to `Tensor::expand(shape)`. ONNX allows a
size-1 dimension to broadcast down to `0`, producing an empty tensor, but burn's
`expand` rejects a target dimension of `0`:

    Broadcast arguments must be positive or -1! Got 0

The Zipa encoder hits this for every input whose subsampled length is even (i.e.
half of them): the "pad to even length" branch expands the last frame by
`ceil(n / 2) * 2 - n`, which is `0` when `n` is already even.

This script rewrites every generated expand block so a zero-containing target
shape yields an explicit empty tensor instead. `Tensor::empty` infers the tensor
kind from the `else` branch, so this works for float, int and bool tensors alike.

Run it after regenerating `zipa-model/src/zipa_small_crctc_ns.rs` with onnx2burn:

    .venv/bin/python python/fix_generated_expand.py \\
        zipa-model/src/zipa_small_crctc_ns.rs
"""

import re
import sys

EXPAND = re.compile(r"^(?P<indent>[ \t]*)(?P<expr>\S.*?)\.expand\(shape\)[ \t]*$")

TEMPLATE = (
    "{indent}if shape.contains(&0) {{\n"
    "{indent}    // ONNX broadcasts a size-1 dim down to 0; burn's expand rejects that.\n"
    "{indent}    Tensor::empty(\n"
    "{indent}        shape.map(|dim| dim as usize),\n"
    "{indent}        (&self.device, {expr}.dtype()),\n"
    "{indent}    )\n"
    "{indent}}} else {{\n"
    "{indent}    {expr}.expand(shape)\n"
    "{indent}}}"
)


def main():
    path = sys.argv[1]
    lines = open(path, encoding="utf-8").read().split("\n")

    output = []
    patched = 0
    for line in lines:
        match = EXPAND.match(line)
        if not match:
            output.append(line)
            continue
        output.append(TEMPLATE.format(**match.groupdict()))
        patched += 1

    open(path, "w", encoding="utf-8").write("\n".join(output))
    print(f"patched {patched} expand blocks in {path}")


if __name__ == "__main__":
    main()
