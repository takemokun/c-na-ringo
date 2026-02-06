use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "ringo-srs",
    about = "SRS learning item manager for ringo",
    long_about = "SRS (Spaced Repetition System) learning item manager for ringo.\n\n\
        Manages English vocabulary learning items with SM-2 algorithm scheduling.\n\
        All commands output JSON to stdout on success (exit 0) and JSON to stderr on failure (exit 1).",
    after_help = "EXAMPLES:\n  \
        ringo-srs add --front \"implement\" --back \"実装する\"\n  \
        ringo-srs list --due --limit 5\n  \
        ringo-srs review item_001 correct good\n  \
        ringo-srs stats"
)]
pub struct Cli {
    /// Path to data file
    #[arg(long, env = "RINGO_SRS_DATA", default_value = "data/learning-items.json")]
    pub data: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new learning item
    #[command(
        long_about = "Add a new learning item.\n\n\
            Required options: --front (English), --back (Japanese)\n\
            Optional options: --type (word|phrase|idiom), --context, --context-ja, --source\n\n\
            Type is auto-detected if omitted: single word → word, multi-word → phrase.\n\
            Duplicate detection is case-insensitive on the --front value.",
        after_help = "EXAMPLES:\n  \
            ringo-srs add --front \"implement\" --back \"実装する\"\n  \
            ringo-srs add --front \"go shopping\" --back \"買い物に行く\" --type phrase\n  \
            ringo-srs add --front \"break the ice\" --back \"場を和ませる\" --type idiom --source ringo-explain"
    )]
    Add {
        /// English word, phrase, or idiom
        #[arg(long)]
        front: String,

        /// Japanese translation
        #[arg(long)]
        back: String,

        /// Item type: word, phrase, or idiom (auto-detected if omitted)
        #[arg(long = "type", value_name = "TYPE")]
        item_type: Option<String>,

        /// Example sentence in English
        #[arg(long)]
        context: Option<String>,

        /// Example sentence in Japanese
        #[arg(long)]
        context_ja: Option<String>,

        /// Source skill that added this item
        #[arg(long)]
        source: Option<String>,
    },

    /// List learning items with optional filters
    #[command(
        long_about = "List learning items with optional filters.\n\n\
            Without flags, returns all items. Filters can be combined.\n\
            Items are returned with id, front, back, type, and context fields.",
        after_help = "EXAMPLES:\n  \
            ringo-srs list                    # all items\n  \
            ringo-srs list --due              # items due for review now\n  \
            ringo-srs list --due --limit 5    # top 5 due items\n  \
            ringo-srs list --weak             # low accuracy items\n  \
            ringo-srs list --status mastered  # mastered items only"
    )]
    List {
        /// Show only items due for review (next_review <= now)
        #[arg(long)]
        due: bool,

        /// Show only weak items (accuracy < 70% and quizzed >= 2 times)
        #[arg(long)]
        weak: bool,

        /// Filter by status: new, learning, reviewing, mastered
        #[arg(long, value_name = "STATUS")]
        status: Option<String>,

        /// Limit number of results returned
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
    },

    /// Record quiz results (single item via args, or batch via stdin JSON)
    #[command(
        long_about = "Record quiz results and update SRS scheduling.\n\n\
            Single-item mode: pass ID, result, and optional difficulty as arguments.\n\
            Batch mode: pipe a JSON array of results via stdin (no arguments).\n\n\
            The SM-2 algorithm calculates the next review date based on the result.",
        after_help = "SINGLE-ITEM MODE:\n  \
            ringo-srs review item_001 correct good\n  \
            ringo-srs review item_002 incorrect\n\n\
          BATCH MODE (stdin JSON array):\n  \
            ringo-srs review <<'EOF'\n  \
            [\n    \
              {\"id\":\"item_001\",\"result\":\"correct\",\"difficulty\":\"good\"},\n    \
              {\"id\":\"item_002\",\"result\":\"incorrect\"}\n  \
            ]\n  \
            EOF"
    )]
    Review {
        /// Item ID (for single-item mode; omit for batch stdin mode)
        #[arg(value_name = "ID")]
        id: Option<String>,

        /// Result: "correct" or "incorrect"
        #[arg(value_name = "RESULT")]
        result: Option<String>,

        /// Difficulty: "easy", "good", or "hard" (only for correct answers)
        #[arg(value_name = "DIFFICULTY")]
        difficulty: Option<String>,
    },

    /// Show learning statistics
    #[command(
        long_about = "Show overall learning statistics.\n\n\
            Returns total items, due count, accuracy percentage,\n\
            breakdowns by status (new/learning/reviewing/mastered)\n\
            and by type (word/phrase/idiom/grammar), plus next due date.",
        after_help = "EXAMPLE:\n  \
            ringo-srs stats"
    )]
    Stats,
}
