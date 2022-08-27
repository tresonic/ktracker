import axios from 'axios';
import authHeader from './auth-header';
if (typeof window !== "undefined") {
  const API_URL = "http://" + window.location.hostname + ":3000/";
  }

class UserService {
  getHighscore() {
    return axios.get(API_URL + 'highscore', { headers: authHeader() });
    // return axios.get(API_URL + 'highscore', {}, { headers: authHeader() });
  }
  getMeters() {
    return axios.get(API_URL + 'get_meters', { headers: authHeader() });
  }
  createEntry(meters) {
    return axios.post(API_URL + 'create_entry', {meters: meters}, { headers: authHeader() });
  }
}
export default new UserService();
