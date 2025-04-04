use md5;

pub fn compute_md5(input: String) -> String {
    let hash = md5::compute(input);
    format!("{:x}", hash)
}
