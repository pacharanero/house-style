# User Interfaces

Reference: the [AI-generated website checklist](https://aitoolpick.org/blog/ai-generated-website-checklist/) as a useful catalogue of generic design tells, and [WCAG 2.2](https://www.w3.org/TR/WCAG22/) for accessibility requirements.

This standard applies to web applications, desktop webviews, public websites, documentation demos, and other interactive visual surfaces. The goal is not to make an interface unusual for its own sake. It is to make it intentional, domain-specific, accessible, and recognisably shaped around the work its users came to do.

## Start with the job

Design from the audience, domain, and repeated workflow before choosing a visual treatment.

- Applications open into a useful workflow, not a marketing landing page explaining that the workflow exists.
- Operational tools such as clinical, administrative, CRM, audit, and developer applications should be quiet, organised, and efficient to scan. Prefer predictable navigation, compact controls, comparison, and repeated action over decorative composition.
- Editorial, branded, portfolio, venue, and game surfaces can be more expressive, but their subject must be visible in the first viewport rather than existing only as a small logo or nav label.
- The first viewport must make the product or page purpose clear and expose the likely next action.
- Use the same domain terms as `CONTEXT.md`, specs, CLI output, and documentation. A UI is not a place to invent friendlier synonyms that split the ubiquitous language.
- Design the frequent path first, then make less common depth easy to reach without crowding the main workflow.

## Layout and hierarchy

Page structure follows the information. Do not apply one template to every section.

- Left-align working content by default. Centre short hero or editorial content only when the composition benefits from it.
- Use proximity to communicate relationships: tighter spacing within a group, wider spacing between groups, and larger breaks only for real changes of subject.
- Do not repeat `heading -> description -> three cards` for several consecutive sections. Mix lists, tables, direct controls, split comparisons, timelines, media, or unframed content when those structures better match the material.
- Consistent components are good. Repeating the same page composition regardless of content is not. Do not vary button, form, or card behaviour merely to look distinctive.
- Cards are for repeated items, modals, and genuinely framed tools. Do not make every page section a floating card and do not put cards inside cards.
- Match type scale to the surface. Hero-scale text belongs in a true hero, not in a dashboard panel, sidebar, dialog, or compact tool.
- Give fixed-format elements such as boards, calendars, grids, toolbars, tiles, counters, and media stable responsive dimensions so dynamic content and hover states cannot shift the layout.
- Text must fit its container at realistic mobile and desktop widths. Wrap it, widen the container, or change the composition rather than clipping or overlapping it.
- A landing-page hero should leave a visible hint of the next section on ordinary mobile and desktop viewports. The hero supports the page; it does not consume the whole product.

## Components and controls

Use familiar controls for familiar jobs and a shared component system for behaviour and appearance.

- Use the established framework or design system already present in the repo. Centralise theme tokens and component variants instead of scattering one-off styles through feature code.
- Use icons from the project's icon library, preferably Lucide where there is no existing choice. Do not use emoji as application icons or draw a new SVG for a familiar symbol.
- Use icon buttons for familiar compact commands such as undo, redo, save, download, zoom, close, and delete. Give unfamiliar icon buttons an accessible name and a visible tooltip on hover/focus.
- Use swatches for colour, segmented controls for modes, toggles or checkboxes for binary settings, sliders/steppers/inputs for numeric values, menus for option sets, tabs for views, and text or icon-plus-text buttons for clear commands.
- Every interaction has the states its user will need: loading, empty, error, disabled, selected, focus, success, and destructive confirmation where applicable.
- Destructive actions state what will be affected and whether recovery is possible. Do not rely on colour alone to distinguish them.
- Prefer progressive disclosure for secondary detail, long option sets, and uncommon configuration. Do not hide the primary workflow behind accordions merely to make the first screen sparse.
- Primary touch controls should normally provide a 44 by 44 CSS pixel target. At minimum, meet WCAG 2.2 AA target size or spacing requirements; inline links and genuinely dense specialist controls use the standard's documented exceptions deliberately.

## Colour, type, and elevation

Choose a visual system deliberately. Do not substitute arbitrary bans or magic counts for design judgement.

- Define semantic colour tokens for background, surface, text, muted text, border, focus, accent, success, warning, and danger. Check text, icon, border, and focus contrast.
- Use a restrained palette, but do not impose a three-colour maximum when semantic states or data visualisation need more. Colour has a job before it has a mood.
- Avoid the default AI/SaaS treatment: purple-to-blue gradients, gradient buttons, decorative gradient orbs, bokeh blobs, and a shadow on every element. A deliberate brand gradient is allowed; an unexplained default is not.
- Prefer borders and spacing for structure in dense tools. Use elevation sparingly and consistently where layers genuinely overlap.
- Avoid one-note palettes dominated by one hue family. Neutral structure plus purposeful accent and semantic colours usually produces clearer hierarchy.
- Choose type for readability, language coverage, product character, and loading behaviour. Common fonts such as Inter, Roboto, and Open Sans are not banned; using any font without an intentional type system is the problem.
- One font family is often enough. Establish hierarchy through size, weight, line-height, and colour; add another family only when it contributes something specific.
- Use tighter line-height for display headings and more generous line-height for prose, while testing actual content rather than enforcing one numeric scale everywhere.
- Keep letter spacing at the font default or `0` unless a brand type system has a tested reason to change it. Negative tracking is not a generic fix for headings.
- Do not scale font size directly with viewport width. Use a stable type scale and responsive layout constraints.

## Interface copy

Copy names the thing, the action, and the consequence.

- Prefer concrete verbs: `Calculate`, `Compare`, `Export attendance`, `Invite teacher`, `Review changes`.
- Buttons describe their action. Avoid generic `Learn more`, `Continue`, or `Submit` when the real action can be named.
- Headings describe the content below them. Replace generic headings such as `Features`, `Solutions`, or `Benefits` with the actual subject.
- Avoid vague promotional language such as `unlock`, `empower`, `seamless`, `revolutionary`, `cutting-edge`, and `elevate` unless the sentence makes a specific, supportable claim.
- Supporting copy should be short enough for its container. Longer explanation belongs in body content, contextual help, docs, or a linked detail view.
- Do not fill application screens with prose explaining the interface. Good labels, sensible defaults, examples, and immediate feedback should carry the workflow.
- Error messages say what failed and what the user can do next. Preserve useful technical detail in logs without exposing secrets or dumping internals into the UI.

## Images and visual assets

Every visual asset must earn its place.

- Use real screenshots, product views, relevant photographs, diagrams, or objects when users need to inspect the actual subject.
- Match an image to the exact content it illustrates. Do not choose generic stock imagery by keyword or reuse one image across unrelated pages.
- Do not use empty gradients, blurred blocks, or decorative placeholders where meaningful media should be. If no useful image exists, design the layout to work without one.
- Generated imagery is allowed when it is appropriate, licensed, and carefully inspected at full resolution. Check anatomy, hands, text, interfaces, logos, lighting, and internal consistency.
- Never present generated imagery as clinical evidence, a real patient, a real product state, or a documentary photograph. Clinical and safety-critical visuals need provenance and domain review.
- Keep image crops informative. Do not obscure the product, place, person, or object behind dark overlays or atmospheric blur when the user needs to see it.

## Motion and feedback

Motion explains change or confirms an action; it is not a tax on every scroll.

- Do not apply scroll-reveal animation to every section or leave content at `opacity: 0` until JavaScript runs.
- Keep transitions short and local. Avoid movement that shifts surrounding content or delays a frequent action.
- Respect `prefers-reduced-motion` and provide a non-moving form for non-essential interaction animation.
- Loading feedback must not resize controls or move the page unexpectedly. Reserve stable space for spinners, progress, validation, and changing labels.
- Animation is never the only signal that state changed.

## Accessibility baseline

Target WCAG 2.2 AA for public and application interfaces.

- Use semantic HTML and native controls before custom widgets.
- Every workflow must be operable by keyboard, with a logical focus order and a clearly visible focus indicator.
- Inputs have persistent labels; errors are associated with their controls; status changes are announced where appropriate.
- Colour is not the only way to communicate state. Text, iconography, shape, or position must carry the same meaning.
- Test text and interface reflow at zoom, not only at the default browser scale.
- Any drag interaction has a non-drag alternative unless dragging is fundamental to the task.
- Autoplay, flashing, parallax, and non-essential motion require particular scrutiny and usually do not belong in operational tools.

## Visual verification

A frontend change is not complete because it compiles. Inspect the rendered interface.

1. Run the application with realistic data, including the longest plausible names and labels.
2. Exercise loading, empty, error, disabled, success, and destructive states, not only the populated happy path.
3. Capture and inspect representative phone, tablet, and desktop screenshots. For responsive web surfaces, useful starting viewports are approximately 390x844, 768x1024, and 1440x900.
4. Check that text, controls, menus, dialogs, media, and fixed navigation do not overlap, clip, overflow, or cover one another.
5. Navigate the main workflow using only the keyboard and inspect visible focus.
6. Test reduced motion and at least 200% browser zoom.
7. Confirm that images, fonts, icons, and canvas/WebGL assets have actually rendered rather than leaving blank or fallback content.
8. Use Playwright screenshots and interaction checks where the repo has browser tests. Keep assertions focused on workflow and layout invariants rather than brittle pixel-perfect snapshots.
9. Review a full-page screenshot with the text mentally removed: does the structure still reflect this product, or is it interchangeable rows of cards?
10. Ask a person unfamiliar with the implementation to identify the page purpose and primary action from the first viewport.

## Generic UI preflight

Before calling an agent-built or rapidly scaffolded interface ready, check:

- No unexplained purple/blue gradient, decorative orb, bokeh background, or gradient CTA.
- No generic fullscreen SaaS hero in front of the actual product.
- No run of identical centred `heading -> subtitle -> three cards` sections.
- No page sections styled as floating cards and no cards nested inside cards.
- No emoji standing in for interface icons.
- No vague headings, buzzword copy, or repeated `Learn more` actions.
- No irrelevant stock image, fake product screenshot, or uninspected generated illustration.
- No animation that hides content, delays work, or ignores reduced-motion preferences.
- No tiny touch targets, invisible focus, clipped labels, overlapping controls, or layout shift during loading.
- No missing empty, error, disabled, or destructive states.

Failing one item is not an automatic design failure. It requires an intentional, product-specific reason rather than a template default.

## Rules this standard deliberately does not make

- It does not ban a particular font.
- It does not require separate heading and body typefaces.
- It does not cap every interface at three colours.
- It does not require giant headings or heroes.
- It does not vary components merely to appear handcrafted.
- It does not ban generated imagery when that imagery is suitable and properly reviewed.
- It does not optimise for an automated "AI-looking" score. Human comprehension, accessibility, and effective work come first.
