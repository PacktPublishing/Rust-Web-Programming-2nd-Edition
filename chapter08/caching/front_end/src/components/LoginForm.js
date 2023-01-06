import React, {Component} from 'react';
import axios from 'axios';
import '../css/LoginForm.css';


class LoginForm extends Component {
    state = {
        username: "",
        password: "",
    }
    submitLogin = (e) => {
        e.preventDefault();
        axios.post("http://localhost:8000/v1/auth/login",
            {"username": this.state.username,
                "password": this.state.password},
            {headers: {"Access-Control-Allow-Origin": "*"}}
        ).then(response => {
            this.setState({username: "", password: ""});
            this.props.handleLogin(response.data["token"]);
        }).catch(error => {
            alert(error);
            this.setState({password: "", firstName: ""});
        });
    }

    handlePasswordChange = (e) => {
        this.setState({password: e.target.value})
    }
    handleUsernameChange = (e) => {
        this.setState({username: e.target.value})
    }
    render() {
        return (<form className="login" onSubmit={this.submitLogin}>
            <h1 className="login-title">Login</h1>
            <input type="text" className="login-input"
                   placeholder="Username"
                   autoFocus onChange={this.handleUsernameChange}
                   value={this.state.username} />
            <input type="password" className="login-input"
                   placeholder="Password" onChange={this.handlePasswordChange}
                   value={this.state.password} />
            <input type="submit" value="Let's Go"
                   className="login-button" />
        </form>)
    }
}
export default LoginForm;
