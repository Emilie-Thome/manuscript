# General remarks:
- [x] add line number in your manuscript so that I can reference the lines.

# Theorem 1 productivity

- [x] the domain of environments over the FINITE set of identifiers
- [x] when you define the syntax, should remark that this set is always finite
- [x] It is suspect that you do not use initialization nor causality. I think that initialization
ensures that last variables/ pre variables are well initialized and causality ensures progress which
means that till all variables are defined, at least one identifier is updated at each run of the
fix-point operator. => they are used...
- [x] Your induction won't stop if you are strictly increasing. => it's written `until all
identifiers are resolved`
- [x] if an iteration fails to resolve any new equations, the remaining undefined identifiers are
mutually dependent. I do not understand well the causality violation. Could you reason on the
distance in the topological order graph to the initial state ? => **use rank**
- [x] what do you mean by the operational semantics is constructive ? => say that **the op sem is
well-defined for every compatible state and input**
- [ ] There are too many repetitions, we should factorize the proof.
