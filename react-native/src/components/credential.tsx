import { colors } from "@/styles/colors";
import { Feather } from "@expo/vector-icons";
import {
  Image,
  ImageBackground,
  Text,
  TouchableOpacity,
  View,
} from "react-native";
import { QRCode } from "./qrcode";

interface CredentialProps {
  image?: string;
  onChangeAvatar?: () => void;
  onExpandQRCode?: () => void;
}

export function Credential({
  onChangeAvatar,
  image,
  onExpandQRCode,
}: CredentialProps) {
  return (
    <View className="w-full items-center self-stretch">
      <Image
        source={require("@/assets/ticket/band.png")}
        className="z-10 h-52 w-24"
      />

      <View className="boder-white=10 mx-3 -mt-5 items-center self-stretch rounded-2xl border bg-black/20 pb-6">
        <ImageBackground
          source={require("@/assets/ticket/header.png")}
          className="h-40 items-center self-stretch overflow-hidden border-b border-white/10 px-6 py-8"
        >
          <View className="w-full flex-row items-center justify-between">
            <Text className="font-bold text-sm text-zinc-50">Unite Summit</Text>
            <Text className="font-bold text-sm text-zinc-50">#123</Text>
          </View>

          <View className="size-40 rounded-full bg-black" />
        </ImageBackground>

        {image ? (
          <TouchableOpacity activeOpacity={0.8} onPress={onChangeAvatar}>
            <Image
              source={{ uri: image }}
              className="-mt-24 size-36 rounded-full"
            />
          </TouchableOpacity>
        ) : (
          <TouchableOpacity
            activeOpacity={0.8}
            className="-mt-24 size-36 items-center justify-center rounded-full bg-gray-400"
            onPress={onChangeAvatar}
          >
            <Feather name="camera" color={colors.green[400]} size={32} />
          </TouchableOpacity>
        )}

        <Text className="mt-4 font-bold text-2xl text-zinc-50">
          IGOR CARDOSO
        </Text>
        <Text className="mb-4 font-bold text-base text-zinc-300">
          igor@cardoso.com
        </Text>

        <QRCode value="teste" size={120} />

        <TouchableOpacity
          activeOpacity={0.7}
          className="mt-6"
          onPress={onExpandQRCode}
        >
          <Text className="font-body text-sm text-orange-500">
            Ampliar QRCode
          </Text>
        </TouchableOpacity>
      </View>
    </View>
  );
}
