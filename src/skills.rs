// ─────────────────────────────────────────────────────────────────────────────
//  ALL SKILL DATA LIVES HERE — edit this file to update the toolkit section.
//  Set `primary: true` on any single item per group to give it the star
//  accent (currently used to mark Python as the primary language).
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct Skill {
    pub name: &'static str,
    pub primary: bool,
}

pub struct SkillGroup {
    pub label: &'static str,
    pub items: &'static [Skill],
}

pub fn all() -> Vec<SkillGroup> {
    vec![
        SkillGroup {
            label: "languages",
            items: &[
                Skill { name: "Python",     primary: true  },
                Skill { name: "TypeScript", primary: false },
                Skill { name: "Java",       primary: false },
                Skill { name: "C++",        primary: false },
                Skill { name: "Rust",       primary: false },
                Skill { name: "SQL",        primary: false },
            ],
        },
        SkillGroup {
            label: "frameworks & data",
            items: &[
                Skill { name: "FastAPI",    primary: false },
                Skill { name: "Next.js",    primary: false },
                Skill { name: "React",      primary: false },
                Skill { name: "PostgreSQL", primary: false },
                Skill { name: "pgvector",   primary: false },
            ],
        },
        SkillGroup {
            label: "cloud & infra",
            items: &[
                Skill { name: "AWS",        primary: false },
                Skill { name: "Azure",      primary: false },
                Skill { name: "Terraform",  primary: false },
                Skill { name: "Docker",     primary: false },
                Skill { name: "Kubernetes", primary: false },
            ],
        },
        SkillGroup {
            label: "workflow",
            items: &[
                Skill { name: "Git",   primary: false },
                Skill { name: "CI/CD", primary: false },
            ],
        },
        // ▼ pre-filled from what's evident in your OTELMIND project.
        //   swap any of these for the specific tools you'd rather list.
        SkillGroup {
            label: "eval & observability",
            items: &[
                Skill { name: "OpenTelemetry",        primary: false },
                Skill { name: "LLM Judges",           primary: false },
                Skill { name: "Cohen's κ Calibration", primary: false },
                Skill { name: "Great Expectations",   primary: false },
                Skill { name: "A/B Testing",          primary: false },
            ],
        },
        SkillGroup {
            label: "agentic & gen ai",
            items: &[
                Skill { name: "LangGraph",        primary: false },
                Skill { name: "LangChain",        primary: false },
                Skill { name: "LlamaIndex",       primary: false },
                Skill { name: "RAG",              primary: false },
                Skill { name: "MCP Server Design", primary: false },
                Skill { name: "Tool Integration", primary: false },
            ],
        },
    ]
}
