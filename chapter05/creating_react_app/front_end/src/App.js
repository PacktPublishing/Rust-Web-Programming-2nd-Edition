import React, { Component } from 'react';

class App extends Component {
  state = {
    "message": "To Do"
  }

  render() {
    return (
        <div className="App">
          <p>{this.state.message} application</p>
        </div>
    )
  }
}

export default App;
