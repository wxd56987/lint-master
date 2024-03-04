import {
  Box,
  Flex,
  Grid,
  Heading,
  Stack,
  Text,
  VStack,
  HStack,
  Button,
  type UseDisclosureReturn,
  ModalFooter,
  ModalBody,
  useBoolean,
  Tooltip,
  Spacer,
  Spinner,
  Center,
  Table,
  TableContainer,
  Tbody,
  Th,
  Thead,
  Tr,
  Td,
  useToast,
  useClipboard,
  Tag,
  TagLeftIcon,
  TagLabel,
} from "@chakra-ui/react";
import { CalendarIcon } from "@chakra-ui/icons";
import { NextSeo } from "next-seo";
import dayjs from "dayjs";
import { useState, useEffect, useCallback, useMemo } from "react";
import {
  VerifiedUser,
  PointsScore,
  Exclamation,
  Rank,
  Crown,
  ArrowRight,
  Participants,
  NYReferal,
  Copy,
  NYEmpty,
} from "@ui/icons";
import { ellipsisString } from "@mcp/web3";
import { profiles } from "@litentry/profiles";
import { useRouter } from "next/router";
import { logEvent } from "@litentry/event-tracking-client";
import scoreBackground from "../public/assets/backgrounds/profile/score-background.png";
import scoreDashboardBackground from "../public/assets/backgrounds/profile/score-dashboard-background.png";
import countDownBackground from "../public/assets/backgrounds/new-year2024/countdown-background.png";
import { useSession } from "../services/useSession";
import { scrollToBottom } from "../services/util";
import { TopNavbar } from "../components/layout/TopNavbar";
import { ConnectWalletButton } from "../components/ConnectWalletButton";
import { Countdown } from "../components/Countdown";
import { Pagination } from "../components/Pagination";
import { Rules } from "../components/NewYearEvent2024/Rules";
import { LightMeUp } from "../components/NewYearEvent2024/LightMeUp";
import { useUserVerifiableCredentials } from "../services/useUserVerifiableCredentials";
import { useUserScore } from "../services/useUserScore";
import { Credential } from "../components/Credentials/Credential";
import { credentialLink } from "../config/routes";
import { PlatformIcon } from "../components/PlatformIcon";
import { Leaderboard } from "../components/NewYearEvent2024/NYLeaderboard";
import { OngoingTag } from "../components/OngoingTag";
import {
  useNYLeaderboardList,
  useUserUploadCredentials,
  useNYLeaderboardUserRank,
} from "../services/useLeaderboard";
import { UIModal } from "../components/UIModal";
import { useTracking } from "../services/useTracking";
import { assembleProfileScore } from "../config/track";
import { targetTimestamp } from "../config/ny2024";
import SEOConfiguration from "../config/next-seo";
import {
  convUnix2UTCTimeString,
  isActivityOngoing,
} from "../services/dayjsUtil";
import {
  CredentialMapForNY,
  useReferenceCode,
  useReportRecommendCode,
  useUserRecommendList,
  useUserReferenceScore,
} from "../services/useNY2024";

import { useSessionDisplay } from "../services/useSessionDisplay";
import type { UserRecommendItem } from "../services/useNY2024";
import type { GetServerSideProps } from "next";
import type { UseMutateFunction } from "react-query";
import type { LeaderboardItem } from "../services/useLeaderboard";

interface ReferralPointsProps
  extends Pick<UseDisclosureReturn, "isOpen" | "onClose"> {
  address?: string;
  shareUrl: string;
  userReferScore: number;
}

type Props = {
  ogImg: string;
};
const profileId = "new-year-2024-campaign";
const profile = profiles[profileId];
export const getServerSideProps: GetServerSideProps<Props> = async (
  context
) => {
  const { query } = context;

  const r = query.r as string;
  let ogImg = "https://idhub.litentry.io/assets/open-graph-image.png";

  if (r) {
    if (r.includes("00000")) {
      ogImg = `https://idhub.litentry.io/assets/open-graph-image-ny.png`;
    } else {
      ogImg = `${process.env.NX_AWS_S3_CDN}ny2024/${r}.png`;
    }
  }

  return { props: { ogImg } };
};

const ReferralPoints = (props: ReferralPointsProps): JSX.Element => {
  const [active, setActive] = useState(true);
  const [pageNum, setPageNum] = useState(10);
  const [pageSize, setPageSize] = useState(1);
  const [total, setTotal] = useState(0);
  const [listType, setListType] = useState<"active" | "pending">("active");
  const { isOpen, onClose, shareUrl, address = "", userReferScore = 0 } = props;
  const { onCopy, hasCopied } = useClipboard(shareUrl);
  const toast = useToast();

  const {
    mutate: fetchUserRecommendList,
    data,
    isLoading,
  } = useUserRecommendList();

  useEffect(() => {
    if (address) {
      fetchUserRecommendList({
        address,
        pageNum,
        pageSize,
        listType,
      });
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [address, listType]);

  // TODO: IGNORE sss----react-hooks/exhaustive-deps
  // TODO: sss----react-hooks/exhaustive-deps
  useEffect(() => {
    if (data) {
      setPageNum(data?.pageNum);
      setPageSize(data?.pageSize);
      setTotal(data?.total);
    }
  }, [data]);

  const uesrRecommedActiveList = data?.recommendList || [];

  // TODO: sss
  const activeHeadList = [
    "Referred users",
    "Referral time",
    "Credential points",
    "Gained Point",
  ];
  const pendingHeadList = [
    "Referred users",
    "Time",
    "Users’ points",
    "Gained Point",
  ];

  useEffect(() => {
    if (hasCopied) {
      toast({
        title: "Copied successfully!",
        status: "success",
        duration: 3000,
        isClosable: true,
        position: "top-right",
      });
    }
  }, [hasCopied, toast]);

  const listToRender = active ? activeHeadList : pendingHeadList;
  const isLastRightAligned = active
    ? activeHeadList.length - 1
    : pendingHeadList.length - 1;

  const listBodyToRender = active
    ? uesrRecommedActiveList
    : uesrRecommedActiveList;

  const renderHeadList = (
    items: string[],
    isLastRightAligned: number
  ): JSX.Element[] => {
    return items.map((item, index) => (
      <Th key={index} padding={0}>
        <Text
          color="grey.300"
          fontSize="sm"
          fontWeight="medium"
          textAlign={
            index === items.length - 1 && isLastRightAligned ? "right" : "left"
          }
        >
          {item}
        </Text>
      </Th>
    ));
  };

  const renderBodyList = (items: UserRecommendItem[]): JSX.Element[] => {
    return items.map((item, index) => (
      <Tr key={index}>
        <Td padding={0} h="38px">
          <Text fontSize="sm" color="grey.50">
            {ellipsisString(item.wallet || "", 5)}
          </Text>
        </Td>
        <Td padding={0} h="38px">
          <Text fontSize="sm" color="grey.50">
            {dayjs(item.updateAt).format("MMM DD YYYY")}
          </Text>
        </Td>
        <Td padding={0} h="38px">
          <Text fontSize="sm" color="grey.50">
            {item.credentialPoints} Pts
          </Text>
        </Td>
        <Td padding={0} h="38px">
          <Text fontSize="sm" color="green.300" textAlign="right">
            {item.gainedPoints} Pts
          </Text>
        </Td>
      </Tr>
    ));
  };

  return (
    <UIModal
      isOpen={isOpen}
      isCentered
      onClose={onClose}
      title="Referral points"
      size="1xl"
    >
      <ModalBody>
        <VStack align="baseline" spacing={2}>
          <Text fontSize="48px" color="#FD7FFB" fontWeight="semibold">
            {userReferScore}
          </Text>
          <HStack
            p={3}
            w="full"
            borderRadius="md"
            bg="rgba(55, 114, 255, 0.20)"
            justifyContent="space-between"
          >
            <HStack>
              <Center w={7} h={7} borderRadius="lg" bg="whiteAlpha.200">
                <NYReferal width={14} />
              </Center>
              <Text color="grey.50" fontSize="sm" fontWeight="medium">
                Get extra points by referring your friends!
              </Text>
            </HStack>
          </HStack>
          <VStack
            align="self-start"
            p={3}
            w="full"
            borderRadius="md"
            bg="grey.700"
            spacing={0}
          >
            <Text color="grey.300" fontSize="xs" fontWeight="semibold">
              REFERRAL LINK
            </Text>
            <HStack w="full" justifyContent="space-between">
              <Text
                color="grey.50"
                fontSize="sm"
                fontWeight="medium"
                textAlign="left"
              >
                {shareUrl}
              </Text>
              <Box
                onClick={() => {
                  onCopy();
                  logEvent("referral-link-copied", {});
                }}
                as="button"
              >
                <Copy fillColor="#f8f8f8" width={24} />
              </Box>
            </HStack>
          </VStack>
          <VStack w="full" spacing={0}>
            <Box h={6} />
            <Box width="full" height="1px" bg="grey.500" />
            <Box h={6} />
          </VStack>
          <HStack spacing={1}>
            <Button
              size="md"
              bg={active ? "grey.900" : "transparent"}
              _hover={{ bg: "grey.900" }}
              onClick={() => {
                setActive(true);
                setListType("active");
                setPageSize(1);
              }}
            >
              Active
            </Button>
            <Button
              size="md"
              bg={!active ? "grey.900" : "transparent"}
              _hover={{ bg: "grey.900" }}
              onClick={() => {
                setActive(false);
                setListType("pending");
                setPageSize(1);
              }}
            >
              Pending
            </Button>
          </HStack>

          {!active && (
            <Box
              my={4}
              px={2}
              py={1}
              bg="rgba(237, 137, 54, 0.15)"
              rounded="4px"
            >
              <Text color="orange.300" fontSize="sm">
                These users haven’t generated any credentials yet.
              </Text>
            </Box>
          )}

          {listBodyToRender.length ? (
            <TableContainer overflowY="auto" w="full">
              <Table
                variant="unstyled"
                bg="grey.600"
                style={{
                  borderCollapse: "separate",
                  borderSpacing: "0 4px",
                }}
              >
                <Thead position="sticky" insetBlockStart={0} zIndex={10}>
                  <Tr bg="grey.600">
                    {isLoading ? (
                      <Center w="full" h="100px">
                        <Spinner />
                      </Center>
                    ) : (
                      renderHeadList(listToRender, isLastRightAligned)
                    )}
                  </Tr>
                </Thead>
                <Tbody>
                  {listBodyToRender && renderBodyList(listBodyToRender)}
                </Tbody>
              </Table>
            </TableContainer>
          ) : (
            <VStack alignItems="center" w="full">
              {isLoading ? (
                <Center w="full" h="100px">
                  <Spinner />
                </Center>
              ) : (
                <>
                  <NYEmpty />
                  <Text color="grey.300" fontSize="sm">
                    You haven&apos;t refer anyone yet
                  </Text>
                </>
              )}
            </VStack>
          )}
        </VStack>
      </ModalBody>
      <ModalFooter
        justifyContent="flex-end"
        width="full"
        borderTop="1px solid #2A2D36"
      >
        {!isLoading && (
          <Pagination
            total={total}
            itemsPerPage={pageNum}
            currentPage={pageSize}
            onPageChange={(v) => {
              setPageSize(v);
            }}
          />
        )}
      </ModalFooter>
    </UIModal>
  );
};

function NewYearCampaign(props: Props): JSX.Element {
  const [currentUrl, setCurrentUrl] = useState("");
  const [page, setPage] = useState(1);
  const [shareUrl, setShareUrl] = useState<string>("");
  const [lights, setLights] = useState<number[]>([0, 0, 0]);
  const [openReferralPoints, actionReferralPointsModal] = useBoolean();
  const { data: session } = useSession();
  const sessionDisplay = useSessionDisplay();
  const [userReferScore, setUserReferScore] = useState(0);
  const [upFlag] = useState<number>(
    parseInt((new Date().getTime() / 1000).toString())
  );

  const { score, userVerifiableCredentials } = useUserScore({ profileId });

  const whiteListedProfileCredentials = Object.keys(profile.credentials).filter(
    (key) => !process.env.NX_BLACKLISTED_CREDENTIALS?.includes(key)
  );
  const { data: verifiableCredentials } = useUserVerifiableCredentials({
    filter: "claimed",
  });

  const address = sessionDisplay.rawAddress;

  const router = useRouter();

  useEffect(() => {
    if (verifiableCredentials) {
      const claimedVc = whiteListedProfileCredentials.map((id) => {
        const definitionId = id as keyof typeof profile.credentials;

        const userVc = userVerifiableCredentials.get(definitionId);
        return {
          id: definitionId,
          claimed: Boolean(userVc?.claimed),
        };
      });

      const typeArray = claimedVc.map((item) => {
        if (Object.prototype.hasOwnProperty.call(CredentialMapForNY, item.id)) {
          return {
            ...item,
            type: CredentialMapForNY[item.id],
          };
        }
      });

      const lightsL = typeArray.filter(
        (item) => item?.claimed && item.type === "L"
      );
      const lightsI = typeArray.filter(
        (item) => item?.claimed && item.type === "I"
      );
      const lightsT = typeArray.filter(
        (item) => item?.claimed && item.type === "T"
      );

      setLights([
        lightsL ? (lightsL.length ? 1 : 0) : 0,
        lightsI ? (lightsI.length ? 1 : 0) : 0,
        lightsT ? (lightsT.length ? 1 : 0) : 0,
      ]);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [verifiableCredentials]);

  const { mutate: reportRecommendCode } = useReportRecommendCode();
  const { mutate: fetchRecommendCode } = useReferenceCode();
  const { mutate: fetchUserReferenceScore } = useUserReferenceScore();

  useEffect(() => {
    if (address && router.query.r) {
      reportRecommendCode({
        address,
        invitationCode: (router.query.r as string).includes("00000")
          ? (router.query.r as string).substring(15)
          : (router.query.r as string).substring(10),
      });
    }
  }, [address, reportRecommendCode, router.query.r]);

  useEffect(() => {
    if (address) {
      fetchUserReferenceScore(
        { address },
        {
          onSuccess: (score) => {
            setUserReferScore(+score);
          },
        }
      );
    }
  }, [address, currentUrl, fetchUserReferenceScore]);

  useEffect(() => {
    if (address) {
      fetchRecommendCode(
        { address },
        {
          onSuccess: (code) => {
            setShareUrl(`${currentUrl}?r=00000${upFlag}${code}`);
          },
        }
      );
    }
  }, [address, currentUrl, fetchRecommendCode, upFlag]);

  useEffect(() => {
    const currentUrl = window.location.href;
    setCurrentUrl(currentUrl);
  }, []);

  const {
    mutate: fetchUserRank,
    data: { rank, score: totalScore } = {
      rank: undefined,
      score: undefined,
    },
  } = useNYLeaderboardUserRank({ address, profile_id: profileId }) as {
    mutate: UseMutateFunction;
    data?: LeaderboardItem;
  };

  const { mutate: uploadCredentials } = useUserUploadCredentials();

  const { data: leaderboardData, mutate: fetchNYLeaderboardList } =
    useNYLeaderboardList();

  useEffect(() => {
    fetchNYLeaderboardList({ page, profile_id: profileId });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const assemblyCredentials = useCallback((): Record<string, unknown>[] => {
    const credentials: Record<string, unknown>[] = [];
    verifiableCredentials?.forEach((record, definition) => {
      credentials.push({
        id: definition,
        credential: JSON.parse(record.rawCredentialText),
      });
    });

    return credentials;
  }, [verifiableCredentials]);

  useEffect(() => {
    if (assemblyCredentials().length) {
      uploadCredentials(
        {
          address,
          credentials: JSON.stringify(assemblyCredentials()),
          profile_id: profileId,
        },
        {
          onSuccess: () => {
            fetchUserRank();
          },
        }
      );
    }
  }, [fetchUserRank, assemblyCredentials, address, uploadCredentials]);

  const track = useTracking();

  const trackCredentialNames = useMemo(
    () => whiteListedProfileCredentials,
    [whiteListedProfileCredentials]
  );

  useEffect(() => {
    track(
      assembleProfileScore({
        action: profileId,
        analysisType: "profileItem",
        wallet: address,
        score,
        credentialNames: trackCredentialNames,
      })
    );
    // There is no need for trackCredentialNames as a dependency here
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [address, profileId, score, track]);

  return (
    <>
      <NextSeo
        title="New Year Campaign"
        openGraph={{
          url: currentUrl,
          title: "New Year Campaign | Identity Hub",
          description: SEOConfiguration.description,
          images: [
            {
              url: props.ogImg,
              width: 901,
              height: 471,
              alt: SEOConfiguration.description,
            },
          ],
          site_name: SEOConfiguration.canonical,
        }}
      />
      <TopNavbar hideHeading />
      <Box maxW="7xl" mx="auto" w="full">
        <Box maxW="7xl" mx="auto">
          <VStack
            spacing={8}
            minWidth={{
              base: "full",
              desktop: "xl",
            }}
            width="full"
            align="baseline"
          >
            <Stack
              direction={{
                base: "column",
                tablet: "row",
              }}
              width="full"
              spacing={12}
            >
              <VStack
                align="baseline"
                borderRadius="3xl"
                flexGrow={1}
                bg="grey.800"
                padding={6}
              >
                <Heading variant="headline4">New Year Campaign</Heading>
                <Flex gap={2} alignItems="center">
                  <Text variant="body2bold" color="grey.100" as="span">
                    Litentry
                  </Text>
                  <VerifiedUser />
                </Flex>
                {profile.activityTime && (
                  <Flex gap={2}>
                    <Tag colorScheme="gray" variant="outline" borderRadius={20}>
                      <TagLeftIcon width="16px" as={CalendarIcon} />
                      <TagLabel lineHeight="16px">
                        {convUnix2UTCTimeString(profile.activityTime.startTime)}
                        -{convUnix2UTCTimeString(profile.activityTime.endTime)}
                      </TagLabel>
                    </Tag>
                    {isActivityOngoing(profile.activityTime) && <OngoingTag />}
                  </Flex>
                )}
                <Text variant="caption2" color="grey.100">
                  Litentry wishes all web3 builders a Happy New Year. Follow LIT
                  to reach new heights in Crypto world in 2024.
                </Text>
              </VStack>
              <VStack
                align="left"
                borderRadius="3xl"
                padding={6}
                flexShrink={0}
                position="relative"
                overflow="hidden"
              >
                <Box fontSize="xs" color="grey.50" fontWeight={500}>
                  SCORE
                </Box>
                <Box
                  borderRadius="3xl"
                  width={{
                    base: "full",
                    desktop: 312,
                  }}
                  height={116}
                  padding="5px"
                  overflow="hidden"
                  bg="whiteAlpha.200"
                >
                  <HStack
                    borderRadius="3xl"
                    background={`
                  linear-gradient(153deg, rgba(51.86, 175.31, 138.28, 0.20) 0%, rgba(38, 64, 63, 0) 100%) border-box,
                  linear-gradient(119deg, #23252E 0%, #18191D 100%) border-box,
                  linear-gradient(295deg, rgba(21, 183, 134, 0.42) 4%, rgba(21, 183, 134, 0) 61%, #38B2AC 100%) border-box
                `}
                    justify={"center"}
                    height="full"
                    width="full"
                    borderWidth={2}
                    borderColor="whiteAlpha.200"
                    boxSizing="border-box"
                    position="relative"
                  >
                    <VStack flex={1}>
                      <HStack align="center">
                        <Text as="span" fontSize="xs" color="whiteAlpha.700">
                          Your Points
                        </Text>
                        <Tooltip
                          hasArrow
                          label="Total points is the summation of credential points and referral points."
                          placement="top"
                        >
                          <Box position="relative" zIndex="banner">
                            <Exclamation
                              width={15}
                              fill="rgba(255, 255, 255, 0.64)"
                            />
                          </Box>
                        </Tooltip>
                      </HStack>
                      <HStack>
                        <Text
                          as="span"
                          fontSize="2xl"
                          color={"green.300"}
                          fontWeight={600}
                        >
                          {score + userReferScore}
                        </Text>
                        <PointsScore />
                      </HStack>
                    </VStack>
                    <VStack flex={1}>
                      <HStack align="center">
                        <Text as="span" fontSize="xs" color="whiteAlpha.700">
                          Your Rank
                        </Text>
                        <Tooltip
                          hasArrow
                          label="The rank is based on your total points."
                          placement="top"
                        >
                          <Box position="relative" zIndex="banner">
                            <Exclamation
                              width={15}
                              fill="rgba(255, 255, 255, 0.64)"
                            />
                          </Box>
                        </Tooltip>
                      </HStack>
                      <HStack>
                        <Text
                          as="span"
                          fontSize="2xl"
                          color="blue.500"
                          fontWeight={600}
                        >
                          {rank ? rank : "--"}
                        </Text>
                        <Rank />
                      </HStack>
                    </VStack>
                    <Box
                      position="absolute"
                      top={0}
                      right={0}
                      bottom={0}
                      left={0}
                      m={0}
                      backgroundSize="contain"
                      backgroundRepeat="no-repeat"
                      backgroundPosition="right center"
                      backgroundImage={scoreBackground.src}
                      zIndex="docked"
                    />
                    <HStack
                      position="absolute"
                      left="-2px"
                      bottom="4px"
                      bg="rgba(24, 38, 37, 0.5)"
                      h={5}
                      paddingLeft="6px"
                      borderRadius="sm"
                      spacing={-1}
                      zIndex="docked"
                    >
                      <Text fontSize="xs" color="whiteAlpha.500">
                        Including
                      </Text>
                      <Box w={3} />
                      <Text
                        fontSize="xs"
                        color="#FD7FFB"
                        onClick={() => {
                          logEvent("referral-button-clicked", {
                            clickedButton: "refer pts",
                          });
                          actionReferralPointsModal.on();
                        }}
                        cursor="pointer"
                      >
                        {userReferScore} referral pts
                      </Text>
                      <Box>
                        <ArrowRight width={24} fillColor="#FD7FFB" />
                      </Box>
                    </HStack>
                  </HStack>
                </Box>
                {session?.signedIn ? (
                  <>
                    <Spacer scaleY={24} />
                    <HStack spacing={1} justifyContent="center">
                      <Participants />
                      <Text fontSize="xs" color="grey.300">
                        {leaderboardData?.list.length} participants have joined
                      </Text>
                    </HStack>
                    <Button
                      size="medium"
                      onClick={scrollToBottom}
                      leftIcon={<Crown />}
                    >
                      Leaderboard
                    </Button>
                  </>
                ) : (
                  <ConnectWalletButton />
                )}
                <Box
                  position="absolute"
                  top={0}
                  right={0}
                  bottom={0}
                  left={0}
                  mt={"0!important"}
                  backgroundSize="100% 100%"
                  backgroundRepeat="no-repeat"
                  backgroundPosition="center center"
                  backgroundImage={scoreDashboardBackground.src}
                  zIndex="hide"
                />
              </VStack>
            </Stack>
          </VStack>
          <Box h={6} />
          <Rules handleReferClick={actionReferralPointsModal.on} />
          <Box h={6} />
          <LightMeUp lightStatus={lights}>
            <VStack
              minWidth={{
                base: "full",
                desktop: "xl",
              }}
              bg={{
                base: "none",
                tablet: "grey.800",
              }}
              borderRadius="4xl"
              width="full"
              height="full"
              padding={{
                base: 0,
                tablet: 6,
              }}
              align="baseline"
            >
              {/* NOTE: We decided to hide Rewards functionality as it's not ready or might be invalidated soon in favour to other concept*/}
              {/*<RewardsProgress profileId={profileId} score={score} />*/}
              <>
                <Text variant="body1bold">Credentials</Text>
                <Grid
                  templateColumns={{
                    base: "repeat(1, 1fr)",
                    tablet: "repeat(2, 1fr)",
                    desktop: "repeat(3, 1fr)",
                  }}
                  gap={6}
                  width="full"
                >
                  {whiteListedProfileCredentials.map((id) => {
                    const definitionId = id as keyof typeof profile.credentials;
                    const credentialDefinition =
                      profile.credentials[definitionId];

                    const userVc = userVerifiableCredentials.get(definitionId);

                    return (
                      <Credential
                        variant="score"
                        key={credentialDefinition.id}
                        isTrending={credentialDefinition?.isTrending}
                        toRoute={{
                          pathname: credentialLink(credentialDefinition.id)
                            .path,
                          query: {
                            source: "profile",
                            profile_id: profileId,
                          },
                        }}
                        logo={
                          <PlatformIcon
                            name={credentialDefinition.icon.name}
                            width={credentialDefinition?.icon.iconConfig?.width}
                            padding={
                              credentialDefinition?.icon.iconConfig?.padding
                            }
                          />
                        }
                        title={credentialDefinition.name}
                        description={credentialDefinition.description}
                        claimed={Boolean(userVc?.claimed)}
                        score={userVc?.score ?? 0}
                        range={profile.scoreBuilder[definitionId].range}
                      />
                    );
                  })}
                </Grid>
              </>
            </VStack>
          </LightMeUp>
        </Box>
        {leaderboardData?.list.length ? (
          <>
            <Leaderboard
              list={leaderboardData?.list}
              ranking={{
                address: address || "",
                rank: rank || 0,
                score: totalScore ?? 0,
              }}
              loading={false}
            />
            <Box h={4} />
            <HStack justifyContent="flex-end">
              <Pagination
                total={leaderboardData?.total || 0}
                itemsPerPage={10}
                currentPage={page}
                onPageChange={(v) => {
                  setPage(v);
                  fetchNYLeaderboardList({
                    profile_id: profileId,
                    page: v,
                  });
                }}
              />
            </HStack>
          </>
        ) : leaderboardData?.list.length === 0 ? (
          <></>
        ) : (
          <Center width="full" height={600}>
            <Spinner size="lg" color="blue.500" />
          </Center>
        )}
      </Box>
      <ReferralPoints
        isOpen={openReferralPoints}
        onClose={actionReferralPointsModal.off}
        address={address}
        shareUrl={shareUrl}
        userReferScore={userReferScore}
      />
    </>
  );
}

export default NewYearCampaign;
