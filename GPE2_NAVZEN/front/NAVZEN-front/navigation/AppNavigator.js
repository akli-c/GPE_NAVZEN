import React from "react";
import { NavigationContainer } from "@react-navigation/native";
import LocationScreen from "../screens/LocationScreen";
import TrailsScreen from "../screens/TrailsScreen";
import TrailDetail from "../screens/TrailDetail";
import RegisterScreen from "../screens/RegisterScreen";
import LoginScreen from "../screens/LoginScreen";
import EmergencyContacts from "../screens/EmergencyContacts";
import { createStackNavigator } from "@react-navigation/stack";
import CommunityScreen from "../screens/CommunityScreen";
import CustomHeader from "../components/CustomHeader";
import ProfileScreen from "../screens/ProfileScreen";
import FriendsScreen from "../screens/FriendsScreen";
import FriendRequestsScreen from "../screens/FriendsRequestsScreen";
import GroupDetailsScreen from "../screens/GroupScreen";

const Stack = createStackNavigator();

function App() {
  return (
    <NavigationContainer>
      <Stack.Navigator
        initialRouteName="LoginScreen"
        screenOptions={({ navigation }) => ({
          header: () => <CustomHeader navigation={navigation} />,
        })}
      > 
        {/* with header */}
       
      </Stack.Navigator>
    </NavigationContainer>
  );
}
export default App;
