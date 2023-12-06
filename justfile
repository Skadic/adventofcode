
today:=`date "+%d"`
current_year:=`date "+%Y"`

_list:
  just --list --justfile={{justfile()}}

# Create a new day for Rust 
new_rs day=today year=current_year:
  cargo generate Skadic/aoc_template --name=day{{day}} --destination {{justfile_directory()}}/{{year}}/Rust/
  
