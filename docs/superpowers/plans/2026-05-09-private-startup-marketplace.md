# Private Startup Marketplace Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a private startup funding workflow where businesses submit confidential company materials, admins review them, and approved capital investors can access business plans only after signing an NDA.

**Architecture:** Extend the existing Tauri Rust backend from simple in-memory submissions into a workflow model with startup review status, investor identity, NDA signature state, and gated document access. Keep Leptos UI split into clear dashboard pages: Startups, Review, Investments, Investors, NDA.

**Tech Stack:** Rust, Leptos CSR, Tauri v2 commands, serde models, in-memory store first, later SQLite persistence.

---

## File Structure

- `src-tauri/src/backend.rs`: backend models, state, commands, tests.
- `src/backend_client.rs`: frontend command client types and async command wrappers.
- `src/dashboard_nav.rs`: dashboard page enum and labels.
- `src/main.rs`: Leptos UI pages and routing.
- `styles.css`: dashboard, review, NDA, and gated business plan styling.

## Phase 1: Startup Submission Model

Add private-company fields to submissions:

- Company name
- Founder/contact
- Email
- Story
- Funding stage
- Capital requested
- DUNS number
- Licensing status
- Business plan filename
- Picture filenames
- Review status: `Pending`, `Approved`, `Rejected`

Backend tests:

- Submitted startup starts as `Pending`.
- Submission stores DUNS, licensing, story, plan, and pictures.
- Pending startups do not appear in investor listings.

## Phase 2: Admin Review

Add an admin review page separate from Startups.

Admin can:

- View pending startups
- Open submitted details
- Approve
- Reject

Backend tests:

- Admin can approve a pending startup.
- Admin can reject a pending startup.
- Approved startups appear on investment page.
- Rejected startups do not appear on investment page.

## Phase 3: Investor Access

Add investor sign-in and investor state.

Investor fields:

- Name
- Email
- Firm
- Accredited investor confirmation
- NDA signed status

Backend tests:

- Investor can sign in or be created.
- Investor starts with `nda_signed = false`.
- Investor can see approved company summaries.
- Investor cannot see business plans before NDA signature.

## Phase 4: NDA Gate

Add listed investor NDA flow before confidential plan access.

Investor can:

- Open approved startup profile
- See NDA requirement
- Sign NDA with typed legal name
- Store signed timestamp
- Unlock business plan access after signing

Backend tests:

- Unsigned investor receives denied access for business plan.
- Signed investor receives allowed access for business plan.
- NDA signature stores investor name and timestamp.

## Phase 5: Business Plan And Media Access

Add document/media references for now, then file storage later.

Startup profile shows:

- Company story
- Stage
- DUNS number
- Licensing
- Pictures
- Business plan access button

Access rules:

- Admin can always review submitted materials.
- Investor can view public approved summary.
- Investor can view business plan only after NDA.

Backend tests:

- Admin can view pending business plan metadata.
- Investor can view approved summary without NDA.
- Investor cannot access plan until NDA signed.
- Investor can access plan after NDA signed.

## Phase 6: Connection Requests

Add investor-to-startup connection requests.

Investor can:

- Request connection after NDA
- Include a short message

Admin can:

- View investor interest
- Mark request as contacted

Backend tests:

- Unsigned investor cannot request connection.
- Signed investor can request connection.
- Admin can list connection requests.
- Admin can mark request contacted.

## Phase 7: Persistence

Move app state from memory to SQLite.

Tables:

- `startups`
- `startup_assets`
- `investors`
- `nda_signatures`
- `connection_requests`

Backend tests:

- Startup persists after app restart.
- NDA signature persists after app restart.
- Connection request persists after app restart.

## Recommended Build Order

1. Backend workflow models and tests.
2. Admin review page.
3. Investor sign-in page.
4. NDA gate.
5. Approved startup investment page.
6. Connection requests.
7. SQLite persistence.

## First Implementable Slice

Build this first:

- Private startup submission fields
- Pending review status
- Admin review page
- Approve/reject actions
- Investment page shows only approved startups

This gives the app the core private-marketplace workflow without introducing investor NDA complexity too early.

