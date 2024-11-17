use octocrab::{params::actions::ArchiveFormat, Octocrab};
use std::{error::Error, io::Cursor};
use zip::ZipArchive;

const OWNER: &str = "iggy-rs";
const REPO: &str = "iggy";

struct GithubHandler {
    octocrab: Octocrab,
}

impl GithubHandler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let token =
            std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable not set");
        let octocrab = Octocrab::builder().personal_token(token).build()?;
        Ok(Self { octocrab })
    }
}

// pub async fn download() -> Result<String, Box<dyn Error>> {
//     let octocrab = Octocrab::builder().personal_token(token).build()?;

//     let runs = octocrab
//         .workflows(OWNER, REPO)
//         .list_runs("performance.yml")
//         .status("success")
//         .per_page(1) // Fetch only the latest run
//         .send()
//         .await?
//         .into_iter();

//     if runs.len() == 0 {
//         return Err("No successful workflow runs found.".into());
//     }

//     let last = runs.clone().next().unwrap();
//     let last_run_id = last.id;
//     println!("Latest Workflow Run ID: {}", last_run_id);

//     let artifact_id = octocrab
//         .actions()
//         .list_workflow_run_artifacts(OWNER, REPO, last_run_id)
//         .send()
//         .await?
//         .value
//         .into_iter()
//         .next()
//         .unwrap()
//         .items[0]
//         .id;

//     println!("Artifact ID: {}", artifact_id);
//     // Download the artifact as a ZIP archive
//     let bytes = octocrab
//         .actions()
//         .download_artifact(OWNER, REPO, artifact_id, ArchiveFormat::Zip)
//         .await?;
//     println!("Downloaded bytes length: {}", bytes.len());

//     // Define the output directory (current directory)
//     let output_dir = std::env::current_dir()?;
//     println!("Unzipping to directory: {:?}", output_dir);

//     // Unzip the downloaded bytes into the output directory
//     let cursor = Cursor::new(bytes);
//     let mut zip = ZipArchive::new(cursor)?;

//     for i in 0..zip.len() {
//         let mut file = zip.by_index(i)?;
//         let outpath = output_dir.join(file.mangled_name());

//         if file.name().ends_with('/') {
//             // It's a directory; create it
//             std::fs::create_dir_all(&outpath)?;
//         } else {
//             // It's a file; ensure the parent directory exists
//             if let Some(parent) = outpath.parent() {
//                 std::fs::create_dir_all(parent)?;
//             }
//             // Create and write the file
//             let mut outfile = std::fs::File::create(&outpath)?;
//             std::io::copy(&mut file, &mut outfile)?;
//         }

//         // Optionally, set Unix permissions if applicable
//         #[cfg(unix)]
//         {
//             use std::os::unix::fs::PermissionsExt;

//             if let Some(mode) = file.unix_mode() {
//                 std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
//             }
//         }
//     }

//     // Assuming the ZIP contains a single top-level folder, return its name
//     // Adjust this part if your ZIP structure is different
//     let output_dir = zip
//         .by_index(0)
//         .unwrap()
//         .name()
//         .split('/')
//         .next()
//         .map(|s| s.to_owned())
//         .unwrap();

//     // // Fallback: return the output directory path
//     // Ok(output_dir.to_string_lossy().to_string())

//     Ok(output_dir)
// }
