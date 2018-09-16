{:ok, tree} = Sled.open("/tmp/sled")

:ok = Sled.del(tree, "hello")
:ok = Sled.set(tree, "hello", "world")

Benchee.run(%{
  "Sled.get/3" => fn -> Sled.get(tree, "hello") end
})
