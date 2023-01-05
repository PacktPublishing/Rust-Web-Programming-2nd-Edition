import React, { Component } from 'react';
import axios from "axios";
import "../App.css";


class CreateToDoItem extends Component {

    state = {
        title: ""
    }

    createItem = () => {
        axios.post("http://127.0.0.1:8000/v1/item/create/" +
            this.state.title,
            {},
            {headers: {"token": "some_token"}})
            .then(response => {
                this.setState({"title": ""});
                this.props.passBackResponse(response);
            });
    }

    handleTitleChange = (e) => {
        this.setState({"title": e.target.value});
    }

    render() {
        return (
            <div className="inputContainer">
                <input type="text" id="name"
                       placeholder="create to do item"
                       value={this.state.title}
                       onChange={this.handleTitleChange}/>
                <div className="actionButton"
                     id="create-button"
                     onClick={this.createItem}>Create</div>
            </div>
        )
    }
}

export default CreateToDoItem;
