use moon::*;

async fn init() {}

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Counters example")
        .append_to_head("<style>html {
            background-color: black;
            color: lightgray;
        }
        
        #app * {
            padding: 5px;
        }
            
        .button {
            cursor: pointer;
            background-color: darkgreen;
            user-select: none;
        }
        
        .button:hover {
            background-color: green;
        }
        
        .column {
            display: flex;
            flex-direction: column;
        }
        
        .row {
            display: flex;
<<<<<<< HEAD
=======
            align-items: center;
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925
        }
        </style>")
}

async fn up_msg_handler(_: UpMsgRequest) {}

fn main() {
    start!(init, frontend, up_msg_handler).unwrap();
}
