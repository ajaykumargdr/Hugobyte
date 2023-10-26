task(
    kind = "openwhisk",
    action_name = "action-1",
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
    action_name = "action-2",
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
        "action-1" : {
            "input" : "t1-field"
        },
        "action-5" : {
        }
    }
)

task(
    kind = "openwhisk",
    action_name = "action-3",
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
        "action-1" : {
            "input" : "t1-field"
        },
    }
)

task(
    kind = "openwhisk",
    action_name = "action-4",
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
        "action-2" : {
            "input" : "t2-field"
        },
        "action-3" : {
            "input" : "t3-field"
        }
    }
)

task(
    kind = "openwhisk",
    action_name = "action-5",
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