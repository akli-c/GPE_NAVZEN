// App.js
import React from 'react';
import { StyleSheet, View } from 'react-native';
import AppNavigator from './navigation/AppNavigator';
import { EmergencyContactsProvider } from './contexts/EmergencyContactsContext';

export default function App() {
  return (
    <EmergencyContactsProvider>
      <View style={styles.container}>
        <AppNavigator />
      </View>
    </EmergencyContactsProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
  },
});