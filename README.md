# scan-color-fix

Corrects color scans made with the _Pantum M6500W_ scanner (or some others).

When scanning a color image, the scanner sequentially switches between red, green, and blue lighting while the sensor is moving.
As a result, the color channels in the image are shifted by a fraction of a pixel relative to each other creating a rainbow effect at the edges.

The program compensates for this shift. It moves the red channel 1/3 px up and the blue channel 1/3 px down using the ́Lánczos interpolation.

| Before | After |
|:------:|:-----:|
|![before](img/a_x8_orig.png)|![before](img/a_x8_fixed.png)|

## Usage

```sh
scan-color-fix <INPUT> <OUTPUT>
```

Arguments:
- `<INPUT>` — Input png file, use `-` for stdin
- `<OUTPUT>` — Output png file, use `-` for stdout

Options:
- `-h`, `--help` — Print help
- `-V`, `--version` — Print version
