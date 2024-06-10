import { colors } from "@/styles/colors";
import { Feather } from "@expo/vector-icons";
import {
  Image,
  ImageBackground,
  Text,
  TouchableOpacity,
  View,
  useWindowDimensions,
} from "react-native";
import { QRCode } from "./qrcode";
import { BadgeStore } from "@/store/badge-store";
import { MotiView } from "moti";

interface CredentialProps {
  onChangeAvatar?: () => void;
  onExpandQRCode?: () => void;
  data: BadgeStore;
}

export function Credential({
  onChangeAvatar,
  onExpandQRCode,
  data,
}: CredentialProps) {
  const { height } = useWindowDimensions();

  return (
    <MotiView
      className="w-full items-center self-stretch"
      from={{
        opacity: 0,
        translateY: -height,
        rotateZ: "50deg",
        rotateY: "30deg",
        rotateX: "30deg",
      }}
      animate={{
        opacity: 1,
        translateY: 0,
        rotateZ: "0deg",
        rotateY: "0deg",
        rotateX: "0deg",
      }}
      transition={{
        type: "spring",
        damping: 20,
        rotateZ: { damping: 15, mass: 3 },
      }}
    >
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
            <Text className="font-bold text-sm text-zinc-50">
              {data.eventTitle}
            </Text>
            <Text className="font-bold text-sm text-zinc-50">#{data.id}</Text>
          </View>

          <View className="size-40 rounded-full bg-black" />
        </ImageBackground>

        {data.image ? (
          <TouchableOpacity activeOpacity={0.8} onPress={onChangeAvatar}>
            <Image
              source={{ uri: data.image }}
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
          {data.name}
        </Text>
        <Text className="mb-4 font-bold text-base text-zinc-300">
          {data.email}
        </Text>

        <QRCode value={data.checkInUrl} size={120} />

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
    </MotiView>
  );
}
