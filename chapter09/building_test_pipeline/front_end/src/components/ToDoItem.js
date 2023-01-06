import React, { Component } from 'react';
import axios from "axios";
import "../App.css";


class ToDoItem extends Component {

    state = {
        "title": this.props.title,
        "status": this.props.status,
        "button": this.processStatus(this.props.status.status)
    }

    processStatus(status) {
        if (status === "PENDING") {
            return "edit"
        } else {
            return "delete"
        }
    }

    inverseStatus(status) {
        if (status === "PENDING") {
            return "DONE"
        } else {
            return "PENDING"
        }
    }

    sendRequest = () => {
        axios.post("http://127.0.0.1:8000/v1/item/" +
            this.state.button,
            {
                "title": this.state.title,
                "status": this.inverseStatus(this.state.status)
            },
            {headers: {"token": localStorage.getItem("user-token")}})
            .then(response => {
                this.props.passBackResponse(response);
            }).catch(error => {
            if (error.response.status === 401) {
                this.props.logout();
            }
        });
    }


    render() {
        return(
            <div className="itemContainer">
                <p>{this.state.title}</p>
                <div className="actionButton" onClick={this.sendRequest}>{this.state.button}</div>
            </div>
        )
    }
}

export default ToDoItem;
