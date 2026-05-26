// ─────────────────────────────────────────────────────────────────────────────
//  ALL PROJECT DATA LIVES HERE — edit this file to update the site.
//  Three statuses: Shipped, Live, Upcoming.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Shipped,
    Live,
    Upcoming,
    Volunteer,
}

impl Status {
    pub fn label(&self) -> &'static str {
        match self {
            Status::Shipped => "shipped",
            Status::Live => "live",
            Status::Upcoming => "incoming",
            Status::Volunteer => "volunteering",
        }
    }
    pub fn color_var(&self) -> &'static str {
        match self {
            Status::Shipped => "var(--accent-sage)",
            Status::Live => "var(--accent)",
            Status::Upcoming => "var(--accent-amber)",
            Status::Volunteer => "var(--accent-rose)",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Project {
    pub id: &'static str,
    pub name: &'static str,
    pub tagline: &'static str,
    pub description: &'static str,
    pub stack: &'static [&'static str],
    pub link: Option<&'static str>,
    pub repo: Option<&'static str>,
    /// Direct .mp4/.webm URL. Loops muted on hover.
    pub video_src: Option<&'static str>,
    /// Poster image shown before hover.
    pub poster: Option<&'static str>,
    pub status: Status,
    /// e.g. "2026.Q1" — shown as the index label on the card.
    pub stamp: &'static str,
}

/// Edit this list. Each entry renders one card.
/// `video_src` placeholders use Coverr stock loops — swap them for your own
/// product demo .mp4 / .webm files (host on Vercel Blob, S3, or your CDN).
pub fn all() -> Vec<Project> {
    vec![
        // ─────────────────────────────────────────────────────────────────
        Project {
            id: "otelmind",
            name: "OTELMIND",
            tagline: "LLM observability & self-healing ops — closed-loop human feedback",
            description: "Closed-loop HITL system on top of a 5-dimension LLM judge. Lifted \
                Cohen's κ from 0.238 → 0.255 on a 33-case human-labeled gold set via a \
                Postgres score_overrides table capturing 95 reviewer corrections/week (24% \
                override rate, live in prod) and a /api/v1/overrides/retrain endpoint that \
                harvests top-|Δ| corrections into few-shot prompt examples. Benchmarked \
                round-robin vs. debate vs. consensus orchestration on Claude Sonnet 4 — \
                uncovered a 3–10× cost differential on identical scenarios. Ships as a \
                6-tool MCP server to Claude Desktop / Claude Code with a CI-gated 95% \
                accuracy floor across 269 parametrized tests.",
            stack: &["python", "fastapi", "postgres", "mcp", "claude", "alembic", "railway"],
            link: Some("https://otelmind-dashboard.vercel.app/traces"),
            repo: None,
            // Static product banner — the real Cost Analytics dashboard.
            // Drop the file at assets/otelmind-cost-analytics.png (Trunk copies
            // the whole assets/ dir to dist/ via the copy-dir link in index.html).
            video_src: None,
            poster: Some("/assets/otelmind-cost-analytics.png"),
            status: Status::Live,
            stamp: "2026.Q2",
        },

        // ─────────────────────────────────────────────────────────────────
        Project {
            id: "querymind",
            name: "QueryMind",
            tagline: "Self-correcting NL→SQL agent with adaptive few-shot memory",
            description: "Three-stage query pipeline: LangChain + GPT-4o generates SQL → \
                sqlglot AST validates safety + EXPLAIN cost-gating → executes against a \
                read-only role and validates rows with Great Expectations. Self-corrects \
                up to 3 times with error context injected. User corrections are stored in \
                Postgres as few-shot examples and re-injected for similar future queries — \
                both the wrong SQL and the fix, so the model learns what to avoid. \
                Defense-in-depth security: SELECT-only enforced structurally (not regex), \
                single-statement guard, statement_timeout backstop, full audit trail.",
            stack: &["python", "langchain", "gpt-4o", "sqlglot", "postgres", "streamlit", "ge"],
            link: Some("https://querymind-frontend-production.up.railway.app/"),
            repo: None,
            // Static product banner — the QueryMind landing page.
            // Drop the file at assets/querymind-landing.png.
            video_src: None,
            poster: Some("/assets/querymind-landing.png"),
            status: Status::Live,
            stamp: "2026.Q1",
        },

        // ─────────────────────────────────────────────────────────────────
        Project {
            id: "cdf-software-dev",
            name: "Software Developer",
            tagline: "Community Dreams Foundation · Science & Technology",
            description: "Volunteer engineering for a nonprofit: AI-assisted document \
                workflows that turn unstructured intake requests into structured outputs \
                with audit-ready decision traces, validation, approval gates, and \
                privacy-conscious handling. 1–2 week iteration cycles with human-in-the-loop \
                review paths so AI outputs stay easy to verify, debug, and improve.",
            stack: &["python", "rag", "langgraph", "azure", "human-in-the-loop"],
            link: None,
            repo: None,
            video_src: None,
            poster: None,
            status: Status::Volunteer,
            stamp: "since 2025.Q4",
        },

        // ─────────────────────────────────────────────────────────────────
        Project {
            id: "android-malware",
            name: "android.malware.dl",
            tagline: "Deep-learning malware detection over extracted Android APIs",
            description: "Published research: a deep-learning pipeline that extracts API \
                call signatures from Android packages and classifies them across known \
                malware families. Compared CNN, LSTM, and hybrid architectures on the \
                Drebin + AMD corpora. Published via Easy Chair, April 2022. Co-authors: \
                Shyam Srujan Mukkamala, Vijay Krishna Kesanapalli, G. Surya Bharthi.",
            stack: &["python", "tensorflow", "keras", "static-analysis"],
            link: Some("https://easychair.org/publications/preprint/G5BK/open"),
            repo: None,
            video_src: None,
            poster: None,
            status: Status::Shipped,
            stamp: "2022.Q2",
        },

        // ─────────────────────────────────────────────────────────────────
        Project {
            id: "next-agent",
            name: "next.agent",
            tagline: "Multi-tenant agent runtime — protocol-aware, budget-guarded",
            description: "Building on OTELMIND's protocol benchmarking: a multi-tenant \
                runtime that picks the cheapest viable orchestration protocol per task \
                (round-robin / debate / consensus) under a per-run dollar cap, with \
                graceful termination + partial result recovery. Targets agent shops that \
                want predictable LLM spend without giving up reasoning quality.",
            stack: &["python", "mcp", "claude", "fastapi", "postgres"],
            link: None,
            repo: None,
            video_src: None,
            poster: None,
            status: Status::Upcoming,
            stamp: "2026.Q3",
        },
    ]
}
