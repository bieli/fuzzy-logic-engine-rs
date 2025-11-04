# fuzzy-logic-engine-rs

![CI status](https://github.com/bieli/fuzzy-logic-engine-rs/actions/workflows/test.yaml/badge.svg)
![github_tag](https://img.shields.io/github/v/tag/bieli/fuzzy-logic-engine-rs)
[![Crates.io](https://img.shields.io/crates/v/fuzzy-logic-engine-rs.svg)](https://crates.io/crates/fuzzy-logic-engine-rs)

A **Rust implementation of a fuzzy inference system (FIS)** — inspired by classical fuzzy logic libraries, but designed with Rust’s safety, performance, and ergonomics in mind.

This crate lets you define **linguistic variables**, **membership functions**, and **rules**, then perform **fuzzy inference** and **defuzzification** to obtain crisp outputs. It’s useful **for decision-making systems, control systems, and AI applications** where reasoning with uncertainty is required.


```ruby
+--------------------------------------+
|   fuzzy_logic_engine_rs              |
|   Rust Fuzzy Inference System Engine |
+--------------------------------------+
```

## Motivations

Fuzzy logic has always fascinated me because it embraces the shades of gray that classical logic often ignores. 
Instead of forcing the world into rigid categories of `true` or `false`, it allows us to capture the subtlety of `"almost," "somewhat," and "mostly"`.
This mirrors how humans naturally think and make decisions, blending intuition with reasoning in a way that feels both elegant and practical.

In mathematics, fuzzy sets reveal a beauty where precision and vagueness coexist, showing that uncertainty can be modeled with rigor.
In engineering and computer science, fuzzy inference systems transform this beauty into real‑world impact, from robotics to finance to everyday decision support.

What excites me most is how fuzzy logic bridges disciplines: it is at once mathematical, philosophical, and deeply human.
It encourages us to see complexity not as a problem to eliminate, but as a richness to embrace and work with.
Every fuzzy rule feels like a small story about the world, capturing experience in a form that machines can understand.

Building this Rust library is my way of celebrating that elegance, while also making it accessible for modern, high‑performance applications.
Ultimately, fuzzy logic reminds us that the world is not binary, and that’s precisely what makes it so endlessly interesting.

## What is Fuzzy Logic?

Fuzzy logic extends classical Boolean logic by allowing **degrees of truth** between 0 and 1.  
Instead of saying *“the service was good”* (true/false), fuzzy logic allows *“the service was 0.7 good and 0.3 average.”*

Key concepts:
- **Linguistic Variable**: A variable described in words (e.g., *temperature*, *speed*, *service quality*).
- **Term**: A fuzzy set describing a linguistic value (e.g., *cold*, *hot*, *excellent*).
- **Membership Function**: Defines how crisp values map to fuzzy degrees (triangle, trapezoid, Gaussian, etc.).
- **Rule**: IF–THEN statements combining fuzzy terms (e.g., *IF service is excellent AND food is good THEN tip is generous*).
- **Defuzzification**: Converts fuzzy results back into a crisp number (e.g., *tip = 17.4%*).

Fuzzy inference systems are widely used in:
- Control systems (washing machines, thermostats, robotics)
- Decision support (recommendation engines, risk analysis)
- AI reasoning under uncertainty
