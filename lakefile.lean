import Lake
open Lake DSL

package lurkFF {
  -- add package configuration options here
}

lean_lib LurkFF {
  -- add library configuration options here
}

@[default_target]
lean_exe lurkFF {
  root := `Main
}
