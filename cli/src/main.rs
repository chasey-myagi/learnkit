use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "learnkit", about = "Personal learning toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the backend server
    Serve {
        #[arg(short, long, default_value = "3377")]
        port: u16,
    },
    /// Initialize a new program workspace
    Init {
        /// Program slug (e.g., "game-dev")
        slug: String,
    },
    /// List all programs
    List,
    /// Show program info
    Info {
        program: String,
    },
    /// Write scope.md to a program
    ScopeWrite {
        program: String,
        #[arg(long)]
        file: String,
    },
    /// Read scope as JSON
    ScopeRead {
        program: String,
    },
    /// Write a lesson HTML (inject template + verify + register)
    LessonWrite {
        program: String,
        subject: String,
        lesson: String,
        #[arg(long)]
        content_file: String,
    },
    /// Verify a lesson HTML integrity
    LessonVerify {
        program: String,
        subject: String,
        lesson: String,
    },
    /// List lessons with optional status filter
    LessonList {
        program: String,
        #[arg(long)]
        status: Option<String>,
    },
    /// Open a lesson in the browser
    LessonOpen {
        program: String,
        subject: String,
        lesson: String,
    },
    /// Get the next unfinished lesson
    Next {
        program: String,
    },
    /// Add a resource to a program
    ResourceAdd {
        program: String,
        url: String,
        #[arg(long, default_value = "doc")]
        r#type: String,
    },
    /// List resources for a program
    ResourceList {
        program: String,
    },
    /// Show learning progress
    Progress {
        program: String,
    },
    /// Update lesson progress status
    ProgressUpdate {
        program: String,
        subject: String,
        lesson: String,
        #[arg(long)]
        status: String,
    },
    /// Check if more lessons need to be prepared
    CheckPrepare {
        program: String,
    },
    /// Write an answer file + record in QA history
    AnswerWrite {
        program: String,
        #[arg(long)]
        request_id: String,
        #[arg(long)]
        lesson: String,
        #[arg(long)]
        selection: String,
        #[arg(long)]
        question: String,
        #[arg(long)]
        answer: String,
    },
    /// Show QA history
    QaHistory {
        program: String,
        #[arg(long)]
        lesson: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { port } => {
            println!("Starting server on port {}...", port);
            // TODO: implement
        }
        Commands::Init { slug } => {
            commands::init::run(&slug)?;
        }
        Commands::List => {
            commands::list::run()?;
        }
        Commands::Info { program } => {
            commands::info::run(&program)?;
        }
        Commands::ScopeWrite { program, file } => {
            commands::scope::write(&program, &file)?;
        }
        Commands::ScopeRead { program } => {
            commands::scope::read(&program)?;
        }
        Commands::LessonWrite { program, subject, lesson, content_file } => {
            commands::lesson::write(&program, &subject, &lesson, &content_file)?;
        }
        Commands::LessonVerify { program, subject, lesson } => {
            commands::lesson::verify(&program, &subject, &lesson)?;
        }
        Commands::LessonList { program, status } => {
            commands::lesson::list(&program, status.as_deref())?;
        }
        Commands::LessonOpen { program, subject, lesson } => {
            commands::lesson::open(&program, &subject, &lesson)?;
        }
        Commands::Next { program } => {
            commands::lesson::next(&program)?;
        }
        Commands::ResourceAdd { program, url, r#type } => {
            commands::resource::add(&program, &url, &r#type)?;
        }
        Commands::ResourceList { program } => {
            commands::resource::list(&program)?;
        }
        Commands::Progress { program } => {
            commands::progress::show(&program)?;
        }
        Commands::ProgressUpdate { program, subject, lesson, status } => {
            commands::progress::update(&program, &subject, &lesson, &status)?;
        }
        Commands::CheckPrepare { program } => {
            commands::progress::check_prepare(&program)?;
        }
        Commands::AnswerWrite { program, request_id, lesson, selection, question, answer } => {
            commands::answer::write(&program, &request_id, &lesson, &selection, &question, &answer)?;
        }
        Commands::QaHistory { program, lesson } => {
            commands::answer::history(&program, lesson.as_deref())?;
        }
    }

    Ok(())
}

mod config;
mod db;
mod commands;
mod scope;
