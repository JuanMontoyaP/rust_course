fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transpose = [[0; 3]; 3];

    for (i, row) in matrix.iter().enumerate() {
        for (j, n) in row.iter().enumerate() {
            transpose[j][i] = *n;
        }
    }

    // for i in 0..3 {
    //     for j in 0..3 {
    //         transpose[j][i] = matrix[i][j];
    //     }
    // }

    transpose
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    dbg!(matrix);
    let transposed = transpose(matrix);
    dbg!(transposed);
}
