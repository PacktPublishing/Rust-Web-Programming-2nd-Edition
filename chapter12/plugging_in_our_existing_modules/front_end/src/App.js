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
      let cachedData = Date.parse(localStorage.getItem("item-cache-date"));
      let now = new Date();
      let difference = Math.round((now - cachedData) / (1000));

      if (difference <= 120) {
          let pendingItems = JSON.parse(localStorage.getItem("item-cache-data-pending"));
          let doneItems = JSON.parse(localStorage.getItem("item-cache-data-done"));
          let pendingItemsCount = pendingItems.length;
          let doneItemsCount = doneItems.length;
          this.setState({
              "pending_items": this.processItemValues(pendingItems),
              "done_items": this.processItemValues(doneItems),
              "pending_items_count": pendingItemsCount,
              "done_items_count": doneItemsCount
          })
      }
      else {
          axios.get(window.location.href + "/v1/item/get",
              {headers: {"token": localStorage.getItem("token")}})
              .then(response => {

                  let pending_items = response.data["pending_items"]
                  let done_items = response.data["done_items"]

                  localStorage.setItem("item-cache-date", new Date());
                  localStorage.setItem("item-cache-data-pending", JSON.stringify(pending_items));
                  localStorage.setItem("item-cache-data-done", JSON.stringify(done_items));

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
  }

  logout() {
      localStorage.removeItem("token");
      this.setState({"login_status": false});
  }

  componentDidMount() {
      let token = localStorage.getItem("token");

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

      localStorage.setItem("item-cache-date", new Date());
      localStorage.setItem("item-cache-data-pending", JSON.stringify(pending_items));
      localStorage.setItem("item-cache-data-done", JSON.stringify(done_items));

      this.setState({
          "pending_items": this.processItemValues(pending_items),
          "done_items": this.processItemValues(done_items),
          "pending_items_count": response.data["pending_item_count"],
          "done_items_count": response.data["done_item_count"]
      })
  }

  handleLogin = (token) => {
      localStorage.setItem("token", token);
      this.setState({"login_status": true});
      this.getItems();
  }

  render() {
    if (this.state.login_status === true) {
        return (
            <div className="App">
                <div className="mainContainer">
                    <div className="header">
                        <p>complete tasks: {this.state.done_items_count}</p>
                        <p>pending tasks: {this.state.pending_items_count}</p>
                    </div>
                    <h1>Pending Items</h1>
                    {this.state.pending_items}
                    <h1>Done Items</h1>
                    {this.state.done_items}
                    <CreateToDoItem passBackResponse={this.handleReturnedState}/>
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
