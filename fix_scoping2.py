with open('src/expand/scoping.rs', 'r') as f:
    content = f.read()

old = 'Expr::Literal(_) =,'
new = 'Expr::Literal(_) =,
            Expr::UnfoldIndex(_) =,'
content = content.replace(old, new, 2)

with open('src/expand/scoping.rs', 'w') as f:
    f.write(content)

print('Done')
