# Vextra

Vexta is a custom declarative UI language for building reactive web interfaces — without touching HTML, CSS, or JavaScript directly.
It’s minimal, high-contrast, and built for people who want control without the clutter of traditional component systems.

## The vision

I’m building Vexta to challenge how we think about front-end development.
I want Vextra to be:

- Declarative: UI is structured like a readable tree.

- Minimal: The output is pure HTML/CSS/JS, but stripped of everything unnecessary.

- Minimalistic ui: Clutter distracts you from the main point.

- Flexible: You can build simple pages, full apps all in one syntax.

- Custom: It’s its own language. Styling, state, and logic live together, but without feeling bloated or coupled.

## Goals 

Custom parser + transpiler from `.vex` files to HTML, CSS, Javascript, all done with a simple command.

## Example Syntax

```vex
app:"Page Title" {
  >div {
    >label:"Hello World"  #{fg:green}
    >label:"Second label" #{fg:red,   size:big}
  }
}
```
The above will get converted to:

```html
<html>
  <head>
    <title>Page Title</title>
  </head>
  <body style="background-color: black; font-family: monospace;">
    <div>
      <span style="color:#8ABB6C;">Hello World</span>
      <span style="color:#E43636;font-size:30px;">Second label</span>
    </div>
  </body>
</html>
```
