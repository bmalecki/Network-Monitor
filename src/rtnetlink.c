// src/hello.c

#include <stdbool.h>
#include <net/if.h>
#include <linux/rtnetlink.h>

#ifndef NDA_RTA
#define NDA_RTA(r) \
    ((struct rtattr *)(((char *)(r)) + NLMSG_ALIGN(sizeof(struct ndmsg))))
#endif

#ifndef NDA_PAYLOAD
#define NDA_PAYLOAD(n) NLMSG_PAYLOAD(n, sizeof(struct ndmsg))
#endif

bool nlmsg_ok(struct nlmsghdr *nlp, int len)
{
    return NLMSG_OK(nlp, len);
}

struct nlmsghdr *nlmsg_next(struct nlmsghdr *nlp, int *len)
{
    return NLMSG_NEXT(nlp, *len);
}

void *nlmsg_data(struct nlmsghdr *nlp)
{
    return NLMSG_DATA(nlp);
}

int rtm_payload(struct nlmsghdr *nlp)
{
    return RTM_PAYLOAD(nlp);
}

int ifa_payload(struct nlmsghdr *nlp)
{
    return IFA_PAYLOAD(nlp);
}

int ifla_payload(struct nlmsghdr *nlp)
{
    return IFLA_PAYLOAD(nlp);
}

int nda_payload(struct nlmsghdr *nlp)
{
    return NDA_PAYLOAD(nlp);
}

struct rtattr *rtm_rta(struct rtmsg *rtp)
{
    return RTM_RTA(rtp);
}

struct rtattr *ifa_rta(struct ifaddrmsg *ifa)
{
    return IFA_RTA(ifa);
}

struct rtattr *ifla_rta(struct ifinfomsg *ifi)
{
    return IFLA_RTA(ifi);
}

struct rtattr *nda_rta(struct ndmsg *ifi)
{
    return NDA_RTA(ifi);
}

bool rta_ok(struct rtattr *rta, int len)
{
    return RTA_OK(rta, len);
}

struct rtattr *rta_next(struct rtattr *atp, int *len)
{
    return RTA_NEXT(atp, *len);
}

const void *rta_data(struct rtattr *atp)
{
    return RTA_DATA(atp);
}