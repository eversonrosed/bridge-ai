fn main() {
  let dds_path = "/mnt/c/Users/davis/IdeaProjects/bridge-ai/dds/src";
  println!("cargo:rustc-link-search={}", dds_path);
  println!("cargo:rustc-link-lib=dds");
}
