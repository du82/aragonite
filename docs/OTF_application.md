--------------------------------------
## OPEN TECHNOLOGY FUND APPLICATION ##
## DRAFT // BOILERPLATE ##
--------------------------------------

# Project Title *
## This project name can be changed if a full proposal is requested.

[v1] Aragonite - Privacy Search Engine

# Describe your project in 1-3 sentences.

----

[v1] A truly secure search engine with zero client-side JavaScript and complete Tor compatibility, allowing censorship-resistant access to information through distributed mirrors. The system can operate fully airgapped, online, or offline, with a transparent infrastructure that users can verify. Its minimal attack surface and flexible deployment options make it uniquely suited for privacy-sensitive environments, conflict zones, and regions experiencing internet restrictions.

----

[v2] Aragonite is a platform for creating censorship-resistant, secure, focused information access systems.

----

# What problem will your project address?
## Describe the problem that exists. What change is needed? How will your project contribute to solving this problem?
https://docs.opentech.fund/otf-application-guidebook/appendix-i-concept-note-guide#what-problem-will-your-project-address

----

[v1] Current search engines force users to choose between functionality and privacy. 'Private' search engines typically rely heavily on JavaScript, creating numerous security vulnerabilities and making them barely usable over Tor. Meanwhile, users in restricted environments need reliable access to information without exposing themselves to surveillance or tracking. This project solves both problems by providing a fully-functional search engine with virtually no attack surface - it uses zero client-side JavaScript, works seamlessly over Tor, can be easily mirrored to bypass censorship, and can even operate offline or air-gapped. Unlike existing solutions that offer privacy through marketing promises, our system provides privacy through fundamental technical design.

----

[v2] Organizations and individuals in high-risk environments need ways to securely access, archive, and distribute focused sets of information without exposing themselves to surveillance or tracking. Current solutions force a choice between accessibility and security - existing search tools either require trust in centralized providers, depend on complex infrastructure that's difficult to maintain, or create significant security vulnerabilities through their technical design. Privacy-focused alternatives typically rely on JavaScript and complex server architectures, making them unreliable over Tor and leaving numerous attack vectors.

Aragonite solves this by providing a platform for creating and distributing secure, focused information access systems. Its minimal attack surface, ability to create custom indexes, and flexible deployment options (including offline and air-gapped operation) enable organizations to build and maintain their own secure information resources. Unlike existing solutions that offer privacy through policy promises, Aragonite provides security through fundamental technical design, verifiable infrastructure, and the ability to operate even when traditional access methods are blocked.

----

# If this project is funded, what form will it take?

() Technological Development

# Give a brief overview of the activities in this project.
## Briefly list key milestones and activities you will carry out.
https://docs.opentech.fund/otf-application-guidebook/appendix-i-concept-note-guide#give-a-brief-overview-of-the-activities-in-this-project
----

[v1] The project focuses on refining and scaling our existing functional prototype. Key activities include:

1. Enhancing the indexing system to expand search coverage while maintaining security and privacy guarantees
2. Developing documentation and tools to simplify mirror deployment by trusted third parties
3. Creating educational materials about the system's security model and verification methods
4. Building relationships with organizations that could benefit from deploying mirrors
5. Implementing and testing additional circumvention techniques for accessing mirrors in restricted environments
6. Establishing metrics to measure successful deployment and usage while preserving user privacy

----

# Are there similar projects that exist already? How is your project different or complementary to those projects?
## What gaps does your project address? Does your project build on these projects? If so, how? Is there opportunity for collaboration? Provide web links if available.

----

[v1] While users can access DuckDuckGo, Mojeek, or SearXNG through Tor, this creates fundamental privacy and usability challenges. These services maintain complex server infrastructures that could potentially log or be compromised, requiring users to trust their privacy promises rather than verifiable technical guarantees. They are also highly centralized - if their domains are blocked or servers compromised, users lose access entirely. Even Tor access can be disrupted by ISP-level blocking. Our distributed mirror system provides genuine resilience - even if Tor is blocked and primary domains are censored, users can still access mirrors through various paths. Unlike SearXNG instances which can be poorly maintained or unstable, our simplified architecture enables mirrors to be reliably operated with minimal overhead. Users can verify exactly what code is running through neofetch, rather than trusting privacy policies or hoping volunteer instances are properly configured. Simply accessing existing search engines through Tor provides a false sense of security, while our system provides verifiable privacy guarantees through its fundamental design and demonstrated reliability through its distributed architecture.

----

[v2] While tools like DuckDuckGo, Mojeek, and SearXNG offer privacy-focused search, Aragonite addresses fundamentally different needs. These services maintain complex infrastructures requiring users to trust their privacy promises rather than verifiable technical guarantees. They are also highly centralized - if their domains are blocked or servers compromised, users lose access entirely. Even Tor access can be disrupted by ISP-level blocking.

Aragonite's distributed architecture provides genuine resilience - even if Tor is blocked and primary domains are censored, users can still access mirrors through various paths. Unlike SearXNG instances which can be poorly maintained or unstable, our simplified architecture enables mirrors to be reliably operated with minimal overhead. Users can verify exactly what code is running through neofetch, rather than trusting privacy policies or hoping volunteer instances are properly configured.

Most significantly, while existing tools are single-purpose search engines, Aragonite is a platform for creating secure information access systems. Organizations can build their own focused indexes of content, maintain searchable archives, and distribute these through the same secure infrastructure. This enables use cases far beyond traditional search - from preserving access to critical information during internet shutdowns to maintaining specialized knowledge bases for high-risk environments.

There's potential for collaboration with existing privacy infrastructure projects around mirror distribution networks and Tor integration. Our minimal attack surface and verified deployment model could also provide lessons for other privacy tools struggling with security and reliability challenges. The project builds on proven concepts from existing search privacy tools while fundamentally rethinking how to provide verifiable security and censorship resistance through architectural design rather than policy promises.

----

# How long do you estimate this project will take?
## This estimate may be revised if a full proposal is requested.

() 6 months to 1 year

# How much funding do you estimate you will need? (In US Dollars) *
## This estimate can be revised if a full proposal is requested.

[v1] $150,000

# Who would benefit from this project?
## Tell us about the people who will use this project? How is it relevant to them?
https://docs.opentech.fund/otf-application-guidebook/appendix-i-concept-note-guide#who-would-benefit-from-this-project

----

[v1] This project primarily benefits users who need reliable, private access to information in challenging environments: journalists and researchers working on sensitive topics, individuals in regions with heavy surveillance or censorship, human rights organizations needing secure search capabilities, and privacy-conscious users who want verifiable rather than promised security. The system's ability to operate offline or through mirrors makes it particularly valuable during internet shutdowns or in conflict zones. Small organizations can easily host verified mirrors, enabling community-driven distribution of access. Privacy-focused libraries and educational institutions can deploy local instances without worrying about complex security configurations. The project's minimal and verifiable architecture also benefits the broader privacy tool ecosystem by demonstrating how to build genuinely secure services without relying on privacy theater. Our existing community of several hundred users has already demonstrated the real-world value for these use cases.

----

[v2] Aragonite serves organizations and individuals who need to maintain secure access to focused information in challenging environments. Human rights organizations can create and distribute verified information archives with genuine privacy guarantees. Journalists and researchers working on sensitive topics can build specialized research databases without compromising security. Communities facing potential internet restrictions can pre-build and mirror, equally between clearnet and Tor, critical information resources before disruptions occur.

The platform particularly benefits groups in regions with heavy surveillance or censorship, who need more than just private search - they need ways to maintain and distribute their own information systems securely. Privacy-focused libraries and educational institutions can deploy local instances with verified security, rather than hoping volunteer instances are properly configured. Small organizations can easily host mirrors of critical information, enabling community-driven distribution of access even when primary channels are blocked.

The project also benefits the broader privacy tool ecosystem by demonstrating how to build genuinely secure services without relying on complex infrastructure or privacy theater. The emphasis on verifiable security and flexible deployment means organizations can trust the platform for critical information needs rather than hoping centralized services remain accessible and trustworthy.

----

# Where are your intended users, or audiences located?

() Global

# Why are you, and your team members, the right people to work on this project?
## Describe your relevant expertise, knowledge, your past work.
https://docs.opentech.fund/otf-application-guidebook/appendix-i-concept-note-guide#why-are-you-and-your-team-members-the-right-people-to-work-on-this-project

----

[v1] Our team combines deep technical expertise with proven operational leadership.

Du Cheng brings a decade of programming and IT security experience, having made meaningful contributions to critical privacy projects including Signal, Element, Status, and Safing. His track record includes building an Android project that achieved 500,000 downloads, demonstrating ability to deliver privacy-focused tools at scale.

Garrett Robinson provides strategic and operational leadership, bringing direct experience from both Visa and high-growth startups. His background in managing complex projects and scaling operations complements the technical foundation.

Our team has already demonstrated successful collaboration through privacyspreadsheet.com, which reached 100,000 unique visitors on launch day and has been recognized by respected privacy advocates and platforms including nowhere.moe and Ezra. This project showcased our ability to create privacy tools that resonate with users while maintaining rigorous technical standards.

The combination of deep technical expertise in privacy and security with proven operational leadership makes our team uniquely qualified to scale this search engine while maintaining its core security principles.

----

# Please upload any supporting documents to your application.
## Accepted file types are .doc, .docx, .odp, .ods, .odt, .pdf, .ppt, .pptx, .rtf, .txt, .xls, .xlsx

>> 

# My application will be dismissed if it does not fit within OTF's mission, values, principles statements. *

() Yes

# I understand that all intellectual property created with support for this application must be openly licensed. *

() Yes

# If my project involves technology development, I agree to undergo a code security audit facilitated by OTF *

() Yes

--------------------------------------

Application [https://apply.opentech.fund/internet-freedom-fund-concept-note/]
Proposal Guide [https://docs.opentech.fund/otf-application-guidebook/appendix-ii-proposal-guide]