import React from "react";
import { View, Text, TouchableOpacity, StyleSheet, Alert } from "react-native";
import { useNavigation } from "@react-navigation/native";
import { Ionicons } from "@expo/vector-icons";
import AsyncStorage from "@react-native-async-storage/async-storage";

const CustomHeader = ({
  title = "Navzen Co",
  showBack = true,
  showProfile = true,
}) => {
  const navigation = useNavigation();

  const handleLogout = async () => {
    Alert.alert("Déconnexion", "Êtes-vous sûr de vouloir vous déconnecter ?", [
      { text: "Annuler", style: "cancel" },
      {
        text: "Déconnexion",
        onPress: async () => {
          await AsyncStorage.removeItem("jwtToken");
          navigation.replace("LoginScreen");
        },
      },
    ]);
  };

  return (
    <View style={styles.header}>
      {/* bouton de retour */}
      {showBack ? (
        <TouchableOpacity
          onPress={() => navigation.goBack()}
          style={styles.iconButton}
        >
          <Ionicons name="arrow-back-outline" size={24} color="#fff" />
        </TouchableOpacity>
      ) : (
        <View style={styles.placeholder} />
      )}

      <Text style={styles.title}>{title}</Text>

      <View style={styles.rightButtons}>
        {/* profile */}
        {showProfile && (
          <TouchableOpacity
            onPress={() => navigation.navigate("Profile")}
            style={styles.iconButton}
          >
            <Ionicons name="person-circle-outline" size={24} color="#fff" />
          </TouchableOpacity>
        )}

        {/* logout */}
        <TouchableOpacity onPress={handleLogout} style={styles.iconButton}>
          <Ionicons name="log-out-outline" size={24} color="#fff" />
        </TouchableOpacity>
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  header: {
    height: 100,
    backgroundColor: "#00a86b",
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    paddingHorizontal: 15,
    paddingTop: 40,
    elevation: 5,
    shadowColor: "#000",
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.15,
    shadowRadius: 3,
  },
  iconButton: {
    padding: 10,
  },
  title: {
    color: "#FFFFFF",
    fontSize: 18,
    fontWeight: "bold",
    textAlign: "center",
  },
  rightButtons: {
    flexDirection: "row",
    alignItems: "center",
  },
  placeholder: {
    width: 40,
  },
});

export default CustomHeader;
