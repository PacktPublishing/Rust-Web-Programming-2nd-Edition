import React, { Component } from 'react';
import axios from 'axios';
import ToDoItem from './components/ToDoItem';
import CreateToDoItem from "./components/CreateToDoItem";
import LoginForm from "./components/LoginForm";

import "./App.css";


class App extends Component {

    state = {
        "pending_items": [],
        "done_items": [],
        "pending_items_count": 0,
        "done_items_count": 0,
        "login_status": false,
    }
    getItems() {
        axios.get("http://127.0.0.1:8000/v1/item/get",
            {headers: {"token": localStorage.getItem("user-token")}})
            .then(response => {
                let pending_items = response.data["pending_items"]
                let done_items = response.data["done_items"]
                this.setState({
                    "pending_items": this.processItemValues(pending_items),
                    "done_items": this.processItemValues(done_items),
                    "pending_items_count": response.data["pending_item_count"],
                    "done_items_count": response.data["done_item_count"]
                })
            }).catch(error => {
            if (error.response.status === 401) {
                this.logout();
            }
        });
    }

    logout() {
        localStorage.removeItem("user-token");
        this.setState({"login_status": false});
    }

    componentDidMount() {
        let token = localStorage.getItem("user-token");

        if (token !== null) {
            this.setState({login_status: true});
            this.getItems();
        }
    }

    processItemValues(items) {
        let itemList = [];
        items.forEach((item, _)=>{
            itemList.push(
                <ToDoItem key={item.title + item.status}
                          title={item.title}
                          status={item.status}
                          passBackResponse={this.handleReturnedState}
                          logout={this.logout}/>
            )
        })
        return itemList
    }

    handleReturnedState = (response) => {
      let pending_items = response.data["pending_items"]
      let done_items = response.data["done_items"]

      this.setState({
          "pending_items": this.processItemValues(pending_items),
          "done_items": this.processItemValues(done_items),
          "pending_items_count": response.data["pending_item_count"],
          "done_items_count": response.data["done_item_count"]
      })
  }

    handleLogin = (token) => {
        localStorage.setItem("user-token", token);
        this.setState({"login_status": true});
        this.getItems();
    }

    render() {
        if (this.state.login_status === true) {
            return (
                <div className="App">
                    <div className="mainContainer">
                        <div className="header">
                            <p>complete tasks:
                                {this.state.done_items_count}</p>
                            <p>pending tasks:
                                {this.state.pending_items_count}</p>
                        </div>
                        <h1>Pending Items</h1>
                        {this.state.pending_items}
                        <h1>Done Items</h1>
                        {this.state.done_items}
                        <CreateToDoItem
                            passBackResponse={this.handleReturnedState}/>
                    </div>
                </div>
            )
        }
        else {
            return (
                <div className="App">
                    <div className="mainContainer">
                        <LoginForm handleLogin={this.handleLogin} />
                    </div>
                </div>
            )
        }
    }
}

export default App;
