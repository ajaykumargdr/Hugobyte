task(
    kind = "openwhisk",
    action_name = "Action1",
    input = {
        "url": "String",
        "owner_key":"String",
        "address":"String",
        "era":"u32"},
    attributes = {
        "chain" : "Westend",
        "operation" : "stakingpayout"
    },
    deps ={
    }
)

task(
    kind = "openwhisk",
    action_name = "Action2",
    input = {
        "url": "String",
        "owner_key":"String",
        "address":"String",
        "era":"u32"},
    attributes = {
        "chain" : "Westend",
        "operation" : "stakingpayout"
    },
    deps ={
        "Action1" : {
            "url" : "t1-field"
        },
        "Action5" : {
        }
    }
)

task(
    kind = "openwhisk",
    action_name = "Action3",
    input = {
        "url": "String",
        "owner_key":"String",
        "address":"String",
        "era":"u32"},
    attributes = {
        "chain" : "Westend",
        "operation" : "stakingpayout"
    },
    deps ={
        "Action1" : {
            "input" : "t1-field"
        },
    }
)

task(
    kind = "openwhisk",
    action_name = "Action4",
    input = {
        "url": "String",
        "owner_key":"String",
        "address":"String",
        "era":"u32"},
    attributes = {
        "chain" : "Westend",
        "operation" : "stakingpayout"
    },
    deps ={
        "Action2" : {
            "input" : "t2-field"
        },
        "Action3" : {
            "input" : "t3-field"
        }
    }
)

task(
    kind = "openwhisk",
    action_name = "Action5",
    input = {
        "url": "String",
        "owner_key":"String",
        "address":"String",
        "era":"u32"},
    attributes = {
        "chain" : "Westend",
        "operation" : "stakingpayout"
    },
    deps ={
    }
)