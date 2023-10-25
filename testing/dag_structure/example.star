# input = {
#     "url" : "String"
#     "owner_key": "String"
# }
 
# properties ={
#     "chain" : "Westend",
#     "operation" : "stakngpayout"
# }
# task1= task(name, openwhisk, input, properties)
# workflow(task1)
 
add_task("task-1", [] )
add_task("task-2", ["task-1"])
add_task("task-3", ["task-1"])
add_task("task-4", ["task-2", "task-3"])

