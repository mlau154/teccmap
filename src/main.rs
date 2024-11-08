use std::{
    io::Write,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    env,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn create_map_file(color_txt_file: impl AsRef<Path>, output_map_file: impl AsRef<Path> + Clone, color_name_to_assign: String, num_points: u8) {
    if num_points < 2 || num_points > 50 {
        panic!("Tecplot only accepts between 2 and 50 points (inclusive) for color maps.");
    }

    // Read the lines from the text file into a 2-D vector of floats
    let mut color_data = Vec::new();
    let lines = lines_from_file(color_txt_file);
    for line in lines {
        let rgb = line.split_whitespace();
        let mut rgb_float = Vec::new();
        for r_g_b in rgb {
            rgb_float.push(r_g_b.parse::<f64>().unwrap());
        }
        color_data.push(rgb_float);
    }

    // Create an evenly spaced array of indices with length
    // equal to num_points. These will be use to choose
    // only a sparse set of rgb values from the text file
    let mut color_data_indices = Vec::new();
    let mut fractions = Vec::new();
    let mut fraction: f64 = 0.0;
    fractions.push(fraction);
    let mut fraction_int: u32 = 0;
    let num_points_float: f64 = num_points as f64;
    let increment: f64 = 1.0 / (num_points_float - 1.0);
    let mut i = 0;
    while i < num_points {
        color_data_indices.push(fraction_int);
        fraction += increment;
        if i < num_points - 1 {
            fractions.push(fraction);
        }
        fraction_int = (255.0 * fraction).round() as u32;
        i += 1;
    }

    // Create the array of header lines to use
    let headers = vec![
        String::from("#!MC 1040\n"),
        String::from("$!CreateColorMap\n"),
        String::from(format!("  Name = '{}'\n", color_name_to_assign)),
        String::from(format!("  NumControlPoints = {}\n", num_points))
    ];

    // Write everything to the map file
    let mut control_point_idx = 1;
    let mut f = File::create(output_map_file.clone()).expect("Unable to create file");
    for header in headers {
        f.write_all(header.as_bytes()).expect("Unable to write data");
    }
    let mut data_idx = 0;
    let mut color_r: u8;
    let mut color_g: u8;
    let mut color_b: u8;
    while data_idx < color_data.len() {
        if !color_data_indices.contains(&(data_idx as u32)) {
            data_idx += 1;
            continue
        }
        color_r = (color_data[data_idx][0] * 255.0) as u8;
        color_g = (color_data[data_idx][1] * 255.0) as u8;
        color_b = (color_data[data_idx][2] * 255.0) as u8;
        let _ = f.write_all(String::from(format!("  ControlPoint {}\n", control_point_idx)).as_bytes());
        let _ = f.write_all(String::from("    {\n").as_bytes());
        let _ = f.write_all(String::from(format!("    ColorMapFraction = {}\n", fractions[control_point_idx - 1])).as_bytes());
        let _ = f.write_all(String::from("    LeadRGB\n").as_bytes());
        let _ = f.write_all(String::from("      {\n").as_bytes());
        let _ = f.write_all(String::from(format!("      R = {}\n", color_r)).as_bytes());
        let _ = f.write_all(String::from(format!("      G = {}\n", color_g)).as_bytes());
        let _ = f.write_all(String::from(format!("      B = {}\n", color_b)).as_bytes());
        let _ = f.write_all(String::from("      }\n").as_bytes());
        let _ = f.write_all(String::from("    TrailRGB\n").as_bytes());
        let _ =f.write_all(String::from("      {\n").as_bytes());
        let _ = f.write_all(String::from(format!("      R = {}\n", color_r)).as_bytes());
        let _ = f.write_all(String::from(format!("      G = {}\n", color_g)).as_bytes());
        let _ = f.write_all(String::from(format!("      B = {}\n", color_b)).as_bytes());
        let _ = f.write_all(String::from("      }\n").as_bytes());
        let _ = f.write_all(String::from("    }\n").as_bytes());
        control_point_idx += 1;
        data_idx += 1;
    }

    println!("Tecplot color map written to {}", output_map_file.as_ref().display());
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 5 {
        panic!("Must supply exactly 4 arguments");
    }
    //let color_txt_file: String = String::from("/mnt/c/users/mlauer2/Documents/dissertation/data/ColorMaps/hawaii.txt");
    //let output_map_file: String = String::from("/mnt/c/users/mlauer2/Documents/dissertation/data/ColorMaps/hawaii2.map");
    //let color_name_to_assign = String::from("cmcrameri - hawaii");
    //let num_points: u8 = 33;
    //create_map_file(color_txt_file, output_map_file, color_name_to_assign, num_points);
    create_map_file(args[1].clone(), args[2].clone(), args[3].clone(), args[4].parse::<u8>().unwrap());
            //rgb_float.push(r_g_b.parse::<f64>().unwrap());
}
