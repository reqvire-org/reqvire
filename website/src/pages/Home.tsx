import { useEffect, useRef } from "react";
import { Link } from "react-router-dom";
import { Footer } from "@/components/Footer";
import {
  Github,
  ArrowRight,
  GitBranch,
  Target,
  Network,
  FileCode,
  Link2,
  CheckCircle,
} from "lucide-react";

function useScrollReveal() {
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add("animate-in");
            observer.unobserve(entry.target);
          }
        });
      },
      { threshold: 0.1 }
    );
    observer.observe(el);
    return () => observer.disconnect();
  }, []);
  return ref;
}

function ScrollReveal({
  children,
  className = "",
  delay = 0,
}: {
  children: React.ReactNode;
  className?: string;
  delay?: number;
}) {
  const ref = useScrollReveal();
  return (
    <div
      ref={ref}
      className={`opacity-0 translate-y-6 transition-all duration-700 ease-out ${className}`}
      style={{ transitionDelay: `${delay}ms` }}
    >
      {children}
    </div>
  );
}

const revealStyle = `
  .animate-in { opacity: 1 !important; transform: translateY(0) !important; }
`;

const knowledgeItems = [
  { term: "Ontologies", desc: "define structural domain vocabulary, relationships, axioms, and semantic meaning." },
  { term: "Concepts", desc: "define curated SKOS thesaurus terms, definitions, synonyms, taxonomy, and concept mappings." },
  { term: "Capabilities", desc: "describe coherent operational, product, business, regulatory, or system abilities." },
  { term: "Requirements", desc: "define implementable obligations, constraints, guarantees, and behavioral expectations." },
  { term: "Contracts", desc: "define requirements in precise terms: source basis, specifications, constraints, behavior, state, interfaces, and input/output semantics." },
  { term: "Verifications", desc: "provide evidence that requirements and capabilities are satisfied." },
  { term: "Implementation artifacts", desc: "connect the model to code, tests, proofs, and generated evidence." },
];

const capabilities = [
  { icon: GitBranch, title: "Semantic Engineering in Git", desc: "Keep engineering knowledge versioned, traceable, reviewable, AI-readable, and semantically connected alongside implementation artifacts." },
  { icon: Target, title: "Capability-Driven Engineering", desc: "Develop systems around coherent operational capabilities instead of disconnected feature lists. Capabilities decompose hierarchically and remain stable across implementation changes." },
  { icon: Network, title: "Ontology-Driven Engineering", desc: "Bring structural ontology and curated concept vocabulary directly into the engineering workflow. Reqvire keeps terminology explicit, reusable, and visible to humans and AI systems." },
  { icon: FileCode, title: "Specification-Driven Development", desc: "Develop from capabilities and requirements while keeping implementation, architecture, tests, and verification artifacts aligned with engineering intent." },
  { icon: Link2, title: "Automated Traceability", desc: "Maintain links between ontologies, capabilities, requirements, contracts, code, tests, proofs, and verification artifacts." },
  { icon: CheckCircle, title: "Verification & Validation", desc: "Track verification coverage, behavioral correctness, implementation alignment, and validation evidence throughout the lifecycle." },
];

const automationItems = [
  "Model diagrams and structural views",
  "Traceability reports",
  "Verification and implementation coverage reports",
  "Change impact reports",
  "Submodel and coupling analysis",
  "Semantic ontology exports",
  "Interactive Explorer UI",
  "MCP tools for AI assistants",
];

const positionTags = [
  "SysML and MBSE",
  "Knowledge graphs",
  "Semantic engineering",
  "Context engineering",
  "AI-native development infrastructure",
];

const nextSteps = [
  { name: "User Guide", href: "/user-guide" },
  { name: "Strategic Vision", href: "/strategic-vision" },
  { name: "Semantic Model", href: "/semantic-model" },
  { name: "Ontologies", href: "/ontologies" },
  { name: "Modeling Language", href: "/modeling-language" },
  { name: "Advanced", href: "/advanced" },
  { name: "Integrations", href: "/integrations" },
];

export default function Home() {
  return (
    <div>
      <style>{revealStyle}</style>

      {/* Hero Section */}
      <section className="relative overflow-hidden rounded-2xl bg-gradient-to-br from-[#0F172A] to-[#1E293B] px-8 sm:px-12 py-16 sm:py-20 mb-20">
        <div className="absolute inset-0 opacity-10">
          <img src="/images/hero-graph.jpg" alt="" className="w-full h-full object-cover" />
        </div>
        <div className="relative z-10 max-w-[640px]">
          <p className="text-xs uppercase tracking-[0.15em] text-white/50 mb-4 animate-fade-in" style={{ animationDelay: "0.1s" }}>
            Semantic Engineering Framework
          </p>
          <h1 className="text-5xl sm:text-6xl font-bold text-white tracking-tight mb-5 animate-fade-in" style={{ animationDelay: "0.2s" }}>
            Reqvire
          </h1>
          <p className="text-lg sm:text-xl font-medium text-white/75 mb-4 animate-fade-in" style={{ animationDelay: "0.4s" }}>
            Build verifiable and traceable software.
          </p>
          <p className="text-base text-white/60 mb-8 max-w-[560px] animate-fade-in" style={{ animationDelay: "0.6s" }}>
            Model ontologies, capabilities, requirements, and verifications in Git, with traceability and engineering context built in for humans, AI agents, and modern engineering workflows.
          </p>
          <div className="flex flex-wrap gap-3 animate-fade-in" style={{ animationDelay: "0.8s" }}>
            <a href="https://github.com/reqvire-org/reqvire" target="_blank" rel="noopener noreferrer"
              className="inline-flex items-center gap-2 px-5 py-2.5 bg-white text-[#0F172A] rounded-md text-sm font-semibold hover:bg-zinc-100 transition-colors">
              <Github className="w-4 h-4" />View it on GitHub
            </a>
            <Link to="/user-guide" className="inline-flex items-center gap-2 px-5 py-2.5 border border-white/25 text-white rounded-md text-sm font-medium hover:bg-white/5 transition-colors">
              Get Started
            </Link>
          </div>
        </div>
      </section>

      {/* Semantic Engineering Framework */}
      <section className="mb-20">
        <ScrollReveal><h2 className="text-3xl font-semibold text-zinc-900 mb-5">Semantic Engineering Framework</h2></ScrollReveal>
        <ScrollReveal delay={100}>
          <p className="text-base text-zinc-600 mb-8 leading-relaxed">
            Reqvire is a semantic engineering framework that turns requirements, capabilities, ontologies, verifications, and code into connected engineering knowledge.
          </p>
        </ScrollReveal>

        <div className="flex flex-col lg:flex-row gap-8 items-start">
          <div className="flex-1 space-y-4">
            {knowledgeItems.map((item, i) => (
              <ScrollReveal key={item.term} delay={150 + i * 80}>
                <div className="flex items-start gap-3">
                  <div className="w-2 h-2 rounded-full bg-blue-500 mt-2 flex-shrink-0" />
                  <p className="text-[15px] leading-relaxed text-zinc-700">
                    <span className="font-semibold text-zinc-900">{item.term}</span> {item.desc}
                  </p>
                </div>
              </ScrollReveal>
            ))}
          </div>
          <ScrollReveal delay={200} className="lg:w-[260px] flex-shrink-0">
            <img src="/images/graph-hierarchy.svg" alt="Semantic engineering model structure" className="w-full rounded-lg border border-zinc-200" />
          </ScrollReveal>
        </div>

        <ScrollReveal delay={400}>
          <div className="mt-8 bg-blue-50/60 border-l-[3px] border-blue-500 rounded-r-lg px-5 py-4">
            <p className="text-[15px] text-zinc-700 leading-relaxed">
              Unlike traditional requirements tools, Reqvire treats engineering knowledge as a connected, versioned model rather than disconnected documents. That knowledge stays reviewable, queryable, and aligned with the codebase.
            </p>
          </div>
        </ScrollReveal>

        <ScrollReveal delay={500}>
          <div className="mt-10">
            <h3 className="text-xl font-semibold text-zinc-900 mb-3">Where Reqvire Sits</h3>
            <p className="text-zinc-600 mb-4">Reqvire naturally sits between:</p>
            <div className="flex flex-wrap gap-2">
              {positionTags.map((tag) => (
                <span key={tag} className="px-3.5 py-1.5 bg-blue-50 text-blue-600 rounded-full text-[13px] font-medium">{tag}</span>
              ))}
            </div>
          </div>
        </ScrollReveal>
      </section>

      {/* Key Capabilities */}
      <section className="bg-zinc-50 rounded-2xl -mx-6 sm:-mx-10 px-6 sm:px-10 py-14 mb-20">
        <ScrollReveal>
          <h2 className="text-3xl font-semibold text-zinc-900 text-center mb-2">Key Capabilities</h2>
          <p className="text-zinc-500 text-center mb-10 max-w-xl mx-auto">
            Six core capabilities that make Reqvire a complete semantic engineering framework.
          </p>
        </ScrollReveal>

        <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-5">
          {capabilities.map((cap, i) => (
            <ScrollReveal key={cap.title} delay={100 + i * 80}>
              <div className="bg-white border border-zinc-200 rounded-xl p-6 hover:shadow-md transition-shadow">
                <div className="w-11 h-11 rounded-full bg-blue-50 flex items-center justify-center mb-4">
                  <cap.icon className="w-5 h-5 text-blue-600" />
                </div>
                <h4 className="font-semibold text-zinc-900 mb-1.5">{cap.title}</h4>
                <p className="text-sm text-zinc-500 leading-relaxed">{cap.desc}</p>
              </div>
            </ScrollReveal>
          ))}
        </div>
      </section>

      {/* Automation */}
      <section className="mb-20">
        <ScrollReveal><h2 className="text-3xl font-semibold text-zinc-900 mb-4">Automation</h2></ScrollReveal>
        <ScrollReveal delay={50}><p className="text-zinc-600 mb-5">Reqvire can generate:</p></ScrollReveal>
        <div className="space-y-2.5">
          {automationItems.map((item, i) => (
            <ScrollReveal key={item} delay={80 + i * 50}>
              <div className="flex items-center gap-3">
                <div className="w-2 h-2 rounded-full bg-blue-500 flex-shrink-0" />
                <span className="text-[15px] text-zinc-700">{item}</span>
              </div>
            </ScrollReveal>
          ))}
        </div>
      </section>

      {/* Next Steps */}
      <section className="mb-10">
        <ScrollReveal><h2 className="text-3xl font-semibold text-zinc-900 mb-5">Next Steps</h2></ScrollReveal>
        <div className="space-y-1.5">
          {nextSteps.map((step, i) => (
            <ScrollReveal key={step.href} delay={50 + i * 50}>
              <Link to={step.href} className="group flex items-center gap-2 text-blue-600 hover:text-blue-700 font-medium py-1.5 transition-colors">
                {step.name}<ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
              </Link>
            </ScrollReveal>
          ))}
        </div>
      </section>

      <Footer />
    </div>
  );
}
