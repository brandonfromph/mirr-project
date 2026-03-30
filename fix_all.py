import sys  
with open('src/expand/rename.rs', 'r') as f: content = f.read()  
lines = content.splitlines()  
new_lines = []  
for line in lines:  
    if 'Literal(_) = in line and 'UnfoldIndex' in line:  
        new_lines.append('            Expr::Literal(_) = 
        new_lines.append('            Expr::UnfoldIndex(_) = 
    elif 'Expr::UnfoldIndex(_) = in line:  
        if not any('UnfoldIndex' in l for l in new_lines[-2:]):  
            new_lines.append(line)  
    else:  
        new_lines.append(line)  
with open('src/expand/rename.rs', 'w') as f: f.write('\n'.join(new_lines))  
print('done') 
