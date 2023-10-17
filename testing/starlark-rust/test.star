'''
# not working

# load('hello_world.star', 'hello_world')
# load("github.com/cirrus-modules/helpers", "task", "container", "script")

# module1 = import_module("./hello_world.star")
# module1.hello() + "world"

'''
a = {
  "hook_type" : "flow",
  "flow_type" : "type of the flow",
  "task_name" : "name of the task",
  "depend_on" : "dependencies" 
}

b = {
  "type" : "value",
  "int" : 5,
  "float" : 3.1,
  "char" : 'c',
  "bool" : True,
  "tuple" : (8, 'j'),
  "list" : ["s", 3, False, 4.4]
}

s = struct(x = 2, y = 3)
s