import os,shutil
def do_split(src,dst,np):
 if not os.path.exists(src):
  print(src+" NOT FOUND")
  return False
 fh=open(src,"r")
 content=fh.read()
 fh.close()
 lines=content.split(chr(10))
 he=0
 for i in range(len(lines)):
  if lines[i].strip().startswith("#[test]"):
   he=i
   break
 h=chr(10).join(lines[:he])
 t=lines[he:]
 n=len(t)
 c=n//np
 if os.path.exists(dst):
  shutil.rmtree(dst)
 os.makedirs(dst)
 mp=os.path.join(dst,"mod.rs")
 fh=open(mp,"w")
 fh.write(h)
 fh.write(chr(10))
 for i in range(np):
  fh.write("mod sub"+str(i+1)+";"+chr(10))
 fh.close()
 for i in range(np):
  s=i*c
  e=(i+1)*c if i<np-1 else n
  pp=os.path.join(dst,"sub"+str(i+1)+".rs")
  fh=open(pp,"w")
  fh.write("use super::*;"+chr(10)+chr(10))
  fh.write(chr(10).join(t[s:e]))
  fh.close()
  print("  sub"+str(i+1)+".rs: "+str(e-s)+" lines")
 os.remove(src)
 print("  deleted "+src)
 return True
files=[("tests/rspu_encoding_extended_tests.rs","tests/rspu_encoding_extended_tests",3),("tests/mega4_totality_verification_tests.rs","tests/mega4_totality_verification_tests",3),("tests/mega3_rspu_verification_tests.rs","tests/mega3_rspu_verification_tests",3),("tests/pattern_tests.rs","tests/pattern_tests",2),("tests/parser_module_extended_tests.rs","tests/parser_module_extended_tests",2)]
for src,dst,np in files:
 print("Splitting "+src+" into "+str(np)+" parts:")
 do_split(src,dst,np)
 print()
print("ALL DONE")
