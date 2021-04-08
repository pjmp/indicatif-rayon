use rayon::prelude::*;

fn main() {
    let multi_bar = indicatif::MultiProgress::new();
    let styl = indicatif::ProgressStyle::default_spinner()
        .template("{spinner} {prefix} {msg} ({elapsed})");

    let tasks = vec!["rng", "syn", "regex"];
    let total = tasks.len();

    println!("with `rayon`");
    tasks.par_iter().enumerate().for_each(|(i, task)| {
        // println!("without `rayon`");
        // tasks.into_iter().enumerate().for_each(|(i, task)| {
        let bar = multi_bar.add(indicatif::ProgressBar::new_spinner());
        bar.set_draw_target(indicatif::ProgressDrawTarget::stdout());
        bar.enable_steady_tick(100);
        bar.set_style(styl.clone());
        bar.set_prefix(&format!("[{}/{}] `{}`", i, total, task));

        bar.set_message(&format!("downloading"));
        std::thread::sleep(std::time::Duration::from_secs(1));

        bar.set_message(&format!("parsing"));
        std::thread::sleep(std::time::Duration::from_secs(1));

        bar.set_message(&format!("saving"));
        std::thread::sleep(std::time::Duration::from_secs(1));
        bar.finish_with_message("Done");
    });

    multi_bar.join().unwrap();
}
