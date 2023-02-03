use cucumber::{given, then, when, World};
use image::DynamicImage;
use image_compare::prelude::*;
use image_compare::Metric;
extern crate image;

// `World` is your shared, likely mutable state.
#[derive(Debug, World, Default)]
pub struct CompareWorld {
    first: Option<DynamicImage>,
    second: Option<DynamicImage>,
    comparison_result: Option<GraySimilarity>,
    comparison_result_rgb: Option<RGBSimilarity>,
    comparison_result_rgba: Option<RGBASimilarity>,
}

#[given(expr = "the images {string} and {string} are loaded")]
fn load_images(world: &mut CompareWorld, first_image: String, second_image: String) {
    let image_one = image::open(first_image).expect("Could not find test-image");
    let image_two = image::open(second_image).expect("Could not find test-image");
    world.first = Some(image_one);
    world.second = Some(image_two);
}

#[when(expr = "comparing the images using RMS as grayscale")]
fn compare_rms(world: &mut CompareWorld) {
    world.comparison_result = Some(
        image_compare::gray_similarity_structure(
            &Algorithm::RootMeanSquared,
            &world.first.as_ref().unwrap().clone().into_luma8(),
            &world.second.as_ref().unwrap().clone().into_luma8(),
        )
        .expect("Error comparing the two images!"),
    );
}

#[when(expr = "comparing the images using RMS as rgb")]
fn compare_rms_rgb(world: &mut CompareWorld) {
    world.comparison_result_rgb = Some(
        image_compare::rgb_similarity_structure(
            &Algorithm::RootMeanSquared,
            &world.first.as_ref().unwrap().clone().into_rgb8(),
            &world.second.as_ref().unwrap().clone().into_rgb8(),
        )
        .expect("Error comparing the two images!"),
    );
}

#[when(expr = "comparing the images using histogram {string} as grayscale")]
fn compare_hist_corr(world: &mut CompareWorld, metric: String) {
    let metric = match metric.as_str() {
        "correlation" => Metric::Correlation,
        "chisquare" => Metric::ChiSquare,
        "intersection" => Metric::Intersection,
        "hellinger distance" => Metric::Hellinger,
        _ => panic!(),
    };
    world.comparison_result = Some(GraySimilarity {
        score: image_compare::gray_similarity_histogram(
            metric,
            &world.first.as_ref().unwrap().clone().into_luma8(),
            &world.second.as_ref().unwrap().clone().into_luma8(),
        )
        .expect("Error comparing the two images!"),
        image: GraySimilarityImage::new(1, 1),
    });
}

#[when(expr = "comparing the images using MSSIM as grayscale")]
fn compare_mssim(world: &mut CompareWorld) {
    world.comparison_result = Some(
        image_compare::gray_similarity_structure(
            &Algorithm::MSSIMSimple,
            &world.first.as_ref().unwrap().clone().into_luma8(),
            &world.second.as_ref().unwrap().clone().into_luma8(),
        )
        .expect("Error comparing the two images!"),
    );
}

#[when(expr = "comparing the images using MSSIM as rgb")]
fn compare_mssim_rgb(world: &mut CompareWorld) {
    world.comparison_result_rgb = Some(
        image_compare::rgb_similarity_structure(
            &Algorithm::MSSIMSimple,
            &world.first.as_ref().unwrap().clone().into_rgb8(),
            &world.second.as_ref().unwrap().clone().into_rgb8(),
        )
        .expect("Error comparing the two images!"),
    );
}
#[when(expr = "comparing the images using the hybrid mode as rgba")]
fn compare_hybrid_rgba(world: &mut CompareWorld) {
    world.comparison_result_rgba = Some(
        image_compare::rgba_hybrid_compare(
            &world.first.as_ref().unwrap().clone().into_rgba8(),
            &world.second.as_ref().unwrap().clone().into_rgba8(),
        )
        .expect("Error comparing the two images!"),
    );
}

#[when(expr = "comparing the images using the hybrid mode as rgb")]
fn compare_hybrid_rgb(world: &mut CompareWorld) {
    world.comparison_result_rgb = Some(
        image_compare::rgb_hybrid_compare(
            &world.first.as_ref().unwrap().clone().into_rgb8(),
            &world.second.as_ref().unwrap().clone().into_rgb8(),
        )
        .expect("Error comparing the two images!"),
    );
}

#[then(expr = "the similarity score is {float}")]
fn check_result_score(world: &mut CompareWorld, score: f64) {
    if world.comparison_result.is_some() {
        assert_eq!(world.comparison_result.as_ref().unwrap().score, score);
    } else if world.comparison_result_rgb.is_some() {
        assert_eq!(world.comparison_result_rgb.as_ref().unwrap().score, score);
    } else if world.comparison_result_rgba.is_some() {
        assert_eq!(world.comparison_result_rgba.as_ref().unwrap().score, score);
    } else {
        panic!("No result calculated yet")
    }
}

#[then(expr = "the similarity image matches {string}")]
fn check_result_image(world: &mut CompareWorld, reference: String) {
    let img = world
        .comparison_result
        .as_ref()
        .unwrap()
        .image
        .to_grayscale();
    let image_one = image::open(reference)
        .expect("Could not find reference-image")
        .into_luma8();
    assert_eq!(
        image_compare::gray_similarity_structure(&Algorithm::RootMeanSquared, &img, &image_one)
            .expect("Could not compare")
            .score,
        1.0
    );
}

#[then(expr = "the rgb similarity image matches {string}")]
fn check_result_image_rgb(world: &mut CompareWorld, reference: String) {
    let img = world
        .comparison_result_rgb
        .as_ref()
        .unwrap()
        .image
        .to_color_map();
    let image_one = image::open(reference)
        .expect("Could not find reference-image")
        .into_rgb8();
    assert_eq!(
        image_compare::rgb_similarity_structure(&Algorithm::RootMeanSquared, &img, &image_one)
            .expect("Could not compare")
            .score,
        1.0
    );
}

#[tokio::main]
async fn main() {
    CompareWorld::run("tests/features/structure_gray.feature").await;
    CompareWorld::run("tests/features/histogram_gray.feature").await;
    CompareWorld::run("tests/features/structure_rgb.feature").await;
    CompareWorld::run("tests/features/hybrid_rgb.feature").await;
    CompareWorld::run("tests/features/hybrid_rgba.feature").await;
}
