# Teccmap
`teccmap` is a small command-line tool written in Rust that is 
used to convert from a text file with space-delimited 
non-dimensional RGB values to the `.map` format required by Tecplot. Assuming that the
executable has been added to the system path (more on this below), `teccmap` can be run
from either Linux or Windows using

`.\teccmap "path/to/txt_file.txt" "path/to/output_file.map" "name displayed in tecplot" <num_points>`

The **first** argument is either a relative or absolute path to a text file containing non-dimensional
RGB values. The contents of the text file should look like

```
0.0 0.5 0.3
0.2 0.88 0.135
0.5 0.23 0.66
0.8 0.3 1.0
1.0 0.6 0.55
```

where each row is a space-delimited vector of RGB values scaled between 0.0 and 1.0. The **second** argument
is a relative or absolute path to the output name of the `.map` file that will be imported into Tecplot.
Note that the quotes are only required for the first two arguments if there are spaces in the path. The
**third** argument is the name as shown in the Tecplot contour colormap dropdown menu. This argument
must be surrounded by quotes if there are spaces.

The **fourth** argument is the number of points that
will be extracted from the text file using a linear spacing. Tecplot requires this value to be between
2 and 50, inclusive. Using the example RGB values shown above, a value of 3 would use the first,
third, and fifth points.

## Import into Tecplot
Once the `.map` file is successfully generated, simply click on the "gear" icon in the "Contour" dialog
of Tecplot and click the "import" option. Alternatively, the map can be added to the Tecplot config file.

## Installation
For a simple installation, first download the executable from

- Windows: `target/x86_64-pc-windows-gnu/release/teccmap.exe`
- Linux/WSL: `target/release/teccmap`

Then, place the executable in any location (e.g., a "Documents" or "Programs" folder) and add this
folder to the system path. To do this:

- Windows: see https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/
- Linux: edit the `~/.bashrc` file to include `export PATH=$PATH:<path/to/installed/exe/location>`
  and then run `source ~/.bashrc`

If you would prefer not to add the executable to the path, just replace `./teccmap` with the
absolute path to the executable.

For an installation that can utilize updates from the Git repository, navigate in the terminal
to a "Documents" or "Programs" folder and run `git clone https://github.com/mlau154/teccmap.git`.
You may have to install Git on Windows if you have never used it before. Then add the
directory containing the executable for your operating system (e.g., 
`C:/Users/<user-name>/Programs/teccmap/target/x86_64-pc-windows-gnu/release`
for Windows or `$HOME/Programs/teccmap/release` for Linux/WSL) to the system path using the
instructions given above.

## Dev
To build for Linux from WSL, use `cargo build --profile=release`. To build for Windows
from WSL, use `cargo build --target=x86_64-pc-windows-gnu --profile=release`.
