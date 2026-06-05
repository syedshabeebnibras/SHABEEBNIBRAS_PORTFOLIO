use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::reveal::observe_reveal;

#[derive(Clone)]
struct Role {
    period: &'static str,
    company: &'static str,
    location: &'static str,
    role: &'static str,
    summary: &'static str,
    bullets: &'static [&'static str],
    tools: &'static [&'static str],
}

fn roles() -> Vec<Role> {
    vec![
        Role {
            period: "mar 2025 — nov 2025",
            company: "DePaul University",
            location: "Chicago, IL · part-time",
            role: "Teaching Assistant · Software Architecture I",
            summary: "Partnered with Prof. Vahid Alizadeh to run a graduate course of \
                40+ students — instruction, grading, office hours, and the rubrics \
                that held it all together.",
            bullets: &[
                "+20% student understanding and +15% engagement by grounding patterns, microservices, scalability, and security in real-world cases through weekly office hours",
                "4-day average feedback turnaround across 150+ submissions — diagrams, design artifacts, and written analyses graded against structured rubrics",
                "60+ one-on-one and group sessions guiding architectural decisions, UML modeling, component responsibility, and trade-off analysis",
                "+25% documentation quality by calibrating rubrics with the lead instructor on risk analysis and non-functional requirements",
            ],
            tools: &["software architecture", "uml", "microservices", "design patterns"],
        },
        Role {
            period: "mar 2022 — jun 2023",
            company: "Wipro Limited",
            location: "Hyderabad, IN",
            role: "Salesforce Administrator & Developer",
            summary: "Owned data ops + admin workflows for a multi-team Salesforce org. \
                Led migrations, built dashboards, and trimmed user-reported issues sprint over sprint.",
            bullets: &[
                "−50% manual errors and ~20 hrs/week saved by leading bulk data ex/imports across departments",
                "−40% system downtime by adding comprehensive program & database testing",
                "Built and maintained custom reports, report types, and dashboards used by ops + sales",
                "Managed roles, profiles, and sharing rules; evaluated each new release for installed packages",
            ],
            tools: &["salesforce", "apex", "soql", "lwc"],
        },
        Role {
            period: "apr 2022 — jun 2022",
            company: "SmartInternz",
            location: "Virtual",
            role: "Salesforce Developer · internship",
            summary: "Trailhead modules and superbadges — Apex, Visualforce, and Lightning Web Components.",
            bullets: &[
                "Completed Salesforce Trailhead modules and superbadges end-to-end",
                "Hands-on experience across Apex, Visualforce, and LWC",
            ],
            tools: &["apex", "visualforce", "lwc"],
        },
        Role {
            period: "jun 2021 — aug 2021",
            company: "Entersoft Security",
            location: "Hyderabad, IN",
            role: "Application Penetration Tester",
            summary: "Web + mobile pentesting on production-grade workloads — \
                from manual analysis to CI-gated regression.",
            bullets: &[
                "Tested 22 production web/mobile workloads with Burp Suite Pro, OWASP ZAP, MobSF, Frida",
                "Found 6 critical + 31 high-severity flaws (SSRF, IDOR, insecure crypto) before release-freeze",
                "Containerized OWASP ZAP regression into Jenkins — MTTD dropped 7 days → 40 minutes",
                "Authored a 20-page remediation playbook + workshops; −72% OWASP Top-10 re-occurrences post-training",
            ],
            tools: &["burp suite", "owasp zap", "mobsf", "frida", "jenkins"],
        },
    ]
}

#[component]
pub fn Experience() -> impl IntoView {
    let list_ref = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        if let Some(el) = list_ref.get() {
            observe_reveal(el.unchecked_into());
        }
    });

    let items = roles();

    view! {
        <section class="section" id="experience">
            <header class="section__head">
                <span class="section__kicker">"01 — Experience"</span>
                <h2 class="section__title">"Where I've been"</h2>
                <p class="section__sub">"four roles across teaching, data, dev, and security — most recent first."</p>
            </header>

            <div class="exp-list" node_ref=list_ref>
                {items.into_iter().enumerate().map(|(i, r)| {
                    view! {
                        <article class="exp" style=format!("--index: {i}")>
                            <header class="exp__head">
                                <span class="exp__when">{r.period}</span>
                                <span class="exp__loc">{r.location}</span>
                            </header>
                            <div class="exp__body">
                                <h3 class="exp__role">{r.role}</h3>
                                <p class="exp__org">{r.company}</p>
                                <p class="exp__summary">{r.summary}</p>
                                <ul class="exp__bullets">
                                    {r.bullets.iter().map(|b| view! { <li>{*b}</li> }).collect::<Vec<_>>()}
                                </ul>
                                <ul class="card__stack exp__tools" aria-label="tools">
                                    {r.tools.iter().map(|t| view! { <li>{*t}</li> }).collect::<Vec<_>>()}
                                </ul>
                            </div>
                        </article>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}
