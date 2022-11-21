use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn laplacian(data: &[u8], out: &mut [u8], width: usize, height: usize) {
    let len = width * height;
    let mut laplacian_image: Vec<i16> = vec![0;len];
    laplacian_gray(data, &mut laplacian_image, width, height);
    for (i, x) in laplacian_image.iter().enumerate() {
        let value = (*x).clamp(0, 255) as u8;
        out[i * 4] = value;
        out[i * 4 + 1] = value;
        out[i * 4 + 2] = value;
    }
}

fn laplacian_gray(data: &[u8], out: &mut [i16], width: usize, height: usize) {
    let kw: usize = 3;
    let half = kw >> 1;
    let kernel_indices = (0..(kw * kw))
        .map(|i| ((i % kw) - half, i / kw - half))
        .collect::<Vec<_>>();

    let kernel: [i16;9] = [0, 1, 0, 1, -4, 1, 0, 1, 0];

    for i in half..(width - half) {
        for j in half..(height - half) {;
            let m: i16 = kernel_indices
                .iter()
                .enumerate()
                .map(|(n, (xk, yk))| data[(i + xk + (j + yk) * width) * 4] as i16 * kernel[n])
                .sum();
            let a = i + j * width;
            // let m_round = m.clamp(0, 255) as u8;
            out[a] = m;
        }
    }

    // remove top and bottom artifacts
    for i in 0..(width) {
        for (aj, bj) in [(0, 1), (height - 1, height - 2)] {
            let a = i + aj * width;
            let b = i + bj * width;
            out[a] = out[b];
        }
    }
    // remove left and right artifacts
    for i in 0..(height) {
        for (aj, bj) in [(0, 1), (width - 1, width - 2)] {
            let a = aj + i * width;
            let b = bj + i * width;
            out[a] = out[b];
        }
    }
}

#[wasm_bindgen]
pub fn variance_of_laplacian(data: &[u8], width: usize, height: usize) -> f32 {
    let len = width * height;
    let mut laplacian_image: Vec<i16> = vec![0;len];
    laplacian_gray(data, &mut laplacian_image, width, height);
    let average = laplacian_image.iter().sum::<i16>() as f32 / len as f32;
    laplacian_image.iter().map(|x| (*x as f32 - average).powi(2)).sum::<f32>() / len as f32
}
