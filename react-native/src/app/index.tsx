import { Alert, Image, View } from "react-native";
import { Input } from "@/components/input";
import { Button } from "@/components/button";
import { MaterialCommunityIcons } from "@expo/vector-icons";
import { colors } from "@/styles/colors";
import { Link, Redirect } from "expo-router";
import { useState } from "react";
import { api } from "@/server/api";
import { useBadgeStore } from "@/store/badge-store";

export default function Home() {
  const [code, setCode] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const badgeStore = useBadgeStore();

  async function handleAccessCredential() {
    if (!code.trim()) {
      return Alert.alert("Ingresso", "Informe o código do ingresso!");
    }

    setIsLoading(true);
    try {
      const { data } = await api.get(`/attendees/${code}/badge`);

      badgeStore.save(data.badge);
    } catch (error) {
      Alert.alert("Ingresso", "Ingresso não encontrado!");
    } finally {
      setIsLoading(false);
    }
  }

  if (badgeStore.data?.checkInUrl) {
    return <Redirect href="/ticket" />;
  }

  return (
    <View className="flex-1 items-center justify-center bg-green-500 p-8">
      <Image
        source={require("@/assets/logo.png")}
        className="h-16"
        resizeMode="contain"
      />
      <View className="mt-12 w-full gap-3">
        <Input>
          <MaterialCommunityIcons
            name="ticket-confirmation-outline"
            color={colors.green[200]}
            size={20}
          />
          <Input.Field
            placeholder="Código do ingresso"
            onChangeText={setCode}
          />
        </Input>

        <Button
          title="Acessar credencial"
          onPress={handleAccessCredential}
          isLoading={isLoading}
        />

        <Link
          href="/register"
          className="mt-8 text-center font-bold text-base text-gray-100"
        >
          Ainda não possui ingresso?
        </Link>
      </View>
    </View>
  );
}
