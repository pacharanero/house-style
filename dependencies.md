# Dependencies

Reference: [Cargo dependency specification](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html), [Cargo resolver recommendations](https://doc.rust-lang.org/cargo/reference/resolver.html), [PyPA dependency specifiers](https://packaging.python.org/specifications/dependency-specifiers/), and [GitHub Dependabot options](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference).

Every dependency is code, a maintenance commitment, and a supply-chain boundary. Add one only when its value exceeds its continuing cost.

## Before Adding A Dependency

Record the concrete problem the dependency solves. Prefer a small standard-library implementation when it is clear, stable, and genuinely cheaper to maintain than another dependency. Do not reimplement a well-tested security primitive, protocol, parser, or domain standard merely to avoid a dependency.

Before admission, review:

- Maintenance: active ownership, recent releases, responsive issue handling, and a credible support path.
- Security: advisories, vulnerability response, release provenance, and whether the package needs privileged access or executes build/install code.
- Licence and provenance: compatibility with the project licence, upstream notices, and whether datasets, generated assets, or bundled code impose extra terms.
- Technical fit: supported platforms, minimum supported runtime/toolchain, feature flags, API stability, and transitive dependency footprint.
- Exit cost: a realistic replacement, removal, or fork path if the project stalls or changes direction.

Use the package's official registry and repository when checking current versions, ownership, releases, and licensing. Do not take an agent's remembered version or a search-result snippet as evidence.

## Direct Dependencies Only

Declare every package the project imports or relies on directly. Do not import or depend on a transitive dependency merely because it happens to be resolved today. Direct declaration makes the contract, provenance, update path, and eventual removal explicit.

Keep dependency roles accurate:

- Runtime dependencies are needed by shipped code.
- Development dependencies are needed only for tests, examples, tooling, or local development.
- Build dependencies execute during builds and deserve the same scrutiny as runtime code.
- Optional features and platform-specific dependencies must be justified and tested in their enabled configurations.

## Versions And Locks

Version requirements and lock files solve different problems.

- Applications, services, CLIs, and deployable tools commit their lockfile and build from it for repeatability. Update it intentionally and review the resolved dependency diff.
- Published libraries declare the compatible ranges they support, not unnecessarily narrow exact pins. Test against their documented minimum supported dependency and toolchain versions where this matters to consumers.
- For Cargo libraries, use the normal caret-compatible requirement with all three version components for most dependencies, such as `serde = "1.0.228"`. Avoid `*`, overly broad ranges, and artificial upper bounds that make downstream resolution fail. Cargo's lockfile controls the exact development/build resolution.
- For Python projects, express project requirements in standard dependency metadata and use the selected tool's committed lockfile or fully pinned generated requirements for deployable environments. Keep the declaration of direct requirements separate from the resolved lock.
- Git dependencies must be deliberate. Pin them to an immutable revision unless tracking a branch is explicitly necessary, and record why a registry release is not suitable.

## Updates And Removal

Keep Dependabot or an equivalent updater configured for every dependency ecosystem actually present. Routine version updates may be grouped and cooled down to reduce noise; security updates must bypass that routine delay.

Treat an update as incoming code:

1. Review manifest and lockfile changes, including new transitive dependencies and changed features.
2. Run the relevant tests, lints, builds, and security checks.
3. Check release notes and migration guidance for majors or security-sensitive packages.
4. Remove dependencies that no longer solve a current problem.

Triage vulnerabilities by exposure and consequence, not a fixed calendar promise. Consider exploitability, reachability, internet exposure, privilege, affected data, known exploitation, available mitigations, and the safety impact. Contain an urgent exposed vulnerability immediately; document any temporary exception, compensating control, owner, and review date.

## Dependency Policy Evidence

For significant additions, security exceptions, private registries, or non-registry sources, record the decision in the PR, an ADR, or the project specification. Include the reason, reviewed version/source, licence/provenance outcome, risks, and any planned re-evaluation.

For clinical software, a dependency that affects clinical behaviour, patient data, authentication, or safety controls may need a hazard-log or safety-case link as well as the engineering review.
