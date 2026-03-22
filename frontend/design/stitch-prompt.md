# LearnKit UI Design Prompt (for Google Stitch)

## App Overview

LearnKit is a personal learning tool. It organizes self-study materials into structured programs with progress tracking. The app has two main pages: a program list (home) and a program detail page (lesson list).

## Visual Style

- **Theme**: Dark mode primary, light mode supported
- **Personality**: Focused, calm, technical but approachable. Like a premium developer tool meets a personal notebook.
- **Color palette**:
  - Primary/Accent: `#6c5ce7` (purple), Secondary: `#a29bfe` (light purple)
  - Dark mode: Background `#0a0a12`, Card `#141425`, Border `#1e1e3a`, Text `#e0e0e8`, Muted text `#7878a0`
  - Light mode: Background `#f5f6fa`, Card `#ffffff`, Border `#d5d5e2`, Text `#1a1a2e`, Muted text `#6a6a85`
  - Status colors: Green `#10ac84` (completed), Yellow `#feca57` (in progress), Blue `#48dbfb` (ready), Gray `#7878a0` (pending), Red `#ff6b6b` (failed)
- **Typography**: System font (SF Pro / Inter style), clean hierarchy
- **Corners**: 10-14px border radius on cards, 999px on pills/badges
- **Shadows**: Minimal, subtle purple glow on hover
- **Spacing**: Generous whitespace, 24-32px padding

## Page 1: Program List (Home)

### Header (sticky top)
- Height: 56px
- Left: Book emoji + "LearnKit" text (the "Learn" part in gradient purple)
- Right: Capsule-shaped day/night toggle with sun and moon icons, the active side has purple highlight
- Bottom: 2px gradient line from purple to light purple, fading to transparent at 70% width
- Clean solid background, no blur effect

### Content Area
- Page title: "我的学习" (My Learning) with a count badge "3 个教程"
- Below: Vertical list of Program Cards with 16px gap

### Program Card
- Large card (full width, ~140px tall), rounded corners 14px
- Layout: Left side has content, right side has a circular SVG progress ring (64px diameter)
- Top-left: Small SVG illustration icon (48px) representing the topic:
  - Game Development → game controller icon
  - Reinforcement Learning → neural network nodes icon
  - Rust Programming → gear/cog icon
- Background: Large semi-transparent version of the same SVG (120px, 6% opacity) in bottom-right corner
- Title: Bold 18px
- Description: One line, muted color, 13px
- Meta row: "3 subjects · 12 lessons · 2026-03-22" in small muted text with dot separators
- Bottom: Gradient-filled progress bar (purple gradient, 6px height, rounded) with percentage text
- The progress ring on the right shows percentage with large bold number inside
- Completed programs: Green progress ring + "✓ 完成" badge
- Hover: Card lifts 3px, purple border glow

### Empty State (when no programs exist)
- Centered in page
- Stacked gradient progress bar decorations as abstract art
- Large text: "开始你的学习之旅"
- Subtitle explaining to use Claude Code to create first program
- Ghost button with purple accent

## Page 2: Program Detail (/program/game-dev)

### Header
Same as Page 1

### Sub-header
- Back arrow + "返回" link on left
- Program title "游戏开发" centered or left-aligned
- Overall progress: "3/12" with gradient progress bar spanning the width

### Subject Groups (Accordion)
Multiple collapsible sections, one per subject (e.g., "游戏设计", "游戏编程", "游戏美术")

**Accordion Header:**
- Click to expand/collapse
- Left: Subject title (bold 15px)
- Center-right: Progress info "2/5 已完成" + small progress bar
- Right: Chevron arrow (rotates 180° when expanded)
- Background: Card color, hover shows subtle purple tint

**Expanded Content — Timeline of Lessons:**
Inside each expanded subject, lessons are displayed as a vertical timeline:

- Left side: 2px vertical line connecting circular nodes
- Each node is 12px circle, color-coded by status:
  - Completed: Solid green circle, line between completed items is also green
  - In Progress: Yellow circle with pulsing glow animation
  - Ready (prepared): Blue circle
  - Pending: Gray circle, 50% opacity
  - Failed: Red circle
- Right of each node, the lesson info:
  - Row 1: Lesson title (14px semibold) + Status badge on right
  - Row 2: Section count + estimated study time (e.g., "3 sections · 预计 15 分钟")
  - For completed lessons: Show actual time (e.g., "实际 23 分钟")
  - For in-progress lessons: Show a thin progress bar below (e.g., "2/3 sections read") + time spent so far

**Status Badges:**
- Ghost/outline style with subtle colored background
- "✓ 已完成" — green tint
- "📖 学习中" — yellow tint, subtle breathing/pulse animation on the dot
- "已备课" — blue tint
- "待备课" — gray, muted
- "失败" — red tint

**Collapsed Subject:**
- Shows only the accordion header
- Chevron points right (→)
- Appears slightly dimmer than expanded subject

### Layout Notes
- First subject is expanded by default (the one user is currently studying)
- Other subjects are collapsed
- Show 2-3 subjects to demonstrate the pattern
- Each lesson row is clickable (cursor pointer, hover highlight)

## Interactions to Show
- Dark mode as the primary view
- One card in hover state (lifted with purple border glow)
- First subject expanded, showing the timeline with mixed statuses
- Second and third subjects collapsed
- The progress ring animation (partial fill)
- The gradient shimmer effect on progress bars

## Do NOT Include
- No sidebar navigation
- No complex menus or dropdowns
- No user avatar or profile section
- No notifications panel
- No search bar (not in this version)
- Keep it minimal and focused
