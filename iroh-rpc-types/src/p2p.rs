include_proto!("p2p");
use futures::stream::BoxStream;

proxy!(
    P2p,
    version: () => VersionResponse => VersionResponse,
    shutdown: () => () => (),
    fetch_bitswap: BitswapRequest => BitswapResponse => BitswapResponse,
    fetch_provider_dht: Key =>
        BoxStream<'static, Result<Providers, tonic::Status>> =>
        BoxStream<'static, anyhow::Result<Providers>> [FetchProviderDhtStream],
    stop_session_bitswap: StopSessionBitswapRequest => () => (),
    notify_new_blocks_bitswap: NotifyNewBlocksBitswapRequest => () => (),
    get_listening_addrs: () => GetListeningAddrsResponse =>  GetListeningAddrsResponse,
    get_peers: () => GetPeersResponse =>  GetPeersResponse,
    peer_connect: ConnectRequest => () => (),
    peer_connect_by_peer_id: ConnectByPeerIdRequest => () => (),
    peer_disconnect: DisconnectRequest => () =>  (),
    lookup: LookupRequest => PeerInfo => PeerInfo,
    gossipsub_add_explicit_peer: GossipsubPeerIdMsg => () =>  (),
    gossipsub_all_mesh_peers: () => GossipsubPeersResponse =>  GossipsubPeersResponse,
    gossipsub_all_peers: () => GossipsubAllPeersResponse =>  GossipsubAllPeersResponse,
    gossipsub_mesh_peers: GossipsubTopicHashMsg => GossipsubPeersResponse =>  GossipsubPeersResponse,
    gossipsub_publish: GossipsubPublishRequest => GossipsubPublishResponse =>  GossipsubPublishResponse,
    gossipsub_remove_explicit_peer: GossipsubPeerIdMsg => () =>  (),
    gossipsub_subscribe: GossipsubTopicHashMsg => GossipsubSubscribeResponse =>  GossipsubSubscribeResponse,
    gossipsub_topics: () => GossipsubTopicsResponse =>  GossipsubTopicsResponse,
    gossipsub_unsubscribe: GossipsubTopicHashMsg => GossipsubSubscribeResponse => GossipsubSubscribeResponse,
    start_providing: Key => () => (),
    stop_providing: Key => () => (),
    local_peer_id: () => PeerIdResponse => PeerIdResponse,
    external_addrs: () => Multiaddrs => Multiaddrs
);
