# NoteCommit

## Message decomposition

$\SinsemillaCommit$ is used in the
[$\NoteCommit$ function](https://zips.z.cash/protocol/protocol.pdf#concretesinsemillacommit).
The input to $\SinsemillaCommit$ is:

$$\DiversifiedTransmitBaseRepr \bconcat
  \DiversifiedTransmitPublicRepr \bconcat
  \ItoLEBSP{64}(\mathsf{v}) \bconcat
  \ItoLEBSP{\BaseLength{Orchard}}(\rho) \bconcat
  \ItoLEBSP{\BaseLength{Orchard}}(\psi),$$

where:
- $\DiversifiedTransmitBaseRepr, \DiversifiedTransmitPublicRepr$ are representations of
  Pallas curve points, with $255$ bits used for the $x$-coordinate and $1$ bit used for
  the $y$-coordinate.
- $\rho, \psi$ are Pallas base field elements.
- $\mathsf{v}$ is a $64$-bit value.
- $\BaseLength{Orchard} = 255.$

Sinsemilla operates on multiples of 10 bits, so we start by decomposing the message into
chunks:

$$
\begin{aligned}
\DiversifiedTransmitBaseRepr &= a \bconcat b_0 \bconcat b_1 \bconcat b_2 \\
 &= (\text{bits 0..=249 of } \mathsf{x(g_d)}) \bconcat
    (\text{bits 250..=253 of } \mathsf{x(g_d)}) \bconcat
    (\text{bit 254 of } \mathsf{x(g_d)}) \bconcat
    (ỹ \text{ bit of } \mathsf{g_d}) \\
\DiversifiedTransmitPublicRepr &= b_3 \bconcat c \bconcat d_0 \bconcat d_1 \\
 &= (\text{bits 0..=3 of } \mathsf{x(pk_d)}) \bconcat
    (\text{bits 4..=253 of } \mathsf{x(pk_d)}) \bconcat
    (\text{bit 254 of } \mathsf{x(pk_d)}) \bconcat
    (ỹ \text{ bit of } \mathsf{pk_d}) \\
\ItoLEBSP{64}(v) &= d_2 \bconcat d_3 \bconcat e_0 \\
 &= (\text{bits 0..=7 of } v) \bconcat
    (\text{bits 8..=57 of } v) \bconcat
    (\text{bits 58..=63 of } v) \\
\ItoLEBSP{\BaseLength{Orchard}}(\rho) &= e_1 \bconcat f \bconcat g_0 \\
 &= (\text{bits 0..=3 of } \rho) \bconcat
    (\text{bits 4..=253 of } \rho) \bconcat
    (\text{bit 254 of } \rho) \\
\ItoLEBSP{\BaseLength{Orchard}}(\psi) &= g_1 \bconcat g_2 \bconcat h_0 \bconcat h_1 \\
 &= (\text{bits 0..=8 of } \psi) \bconcat
    (\text{bits 9..=248 of } \psi) \bconcat
    (\text{bits 249..=253 of } \psi) \bconcat
    (\text{bit 254 of } \psi) \\
\end{aligned}
$$

Then we recompose the chunks into message pieces:

$$
\begin{array}{|c|l|}
\hline
\text{Length (bits)} & \text{Piece} \\\hline
250 & a \\
 10 & b = b_0 \bconcat b_1 \bconcat b_2 \bconcat b_3 \\
250 & c \\
 60 & d = d_0 \bconcat d_1 \bconcat d_2 \bconcat d_3 \\
 10 & e = e_0 \bconcat e_1 \\
250 & f \\
250 & g = g_0 \bconcat g_1 \bconcat g_2 \\
 10 & h = h_0 \bconcat h_1 \bconcat h_2 \\\hline
\end{array}
$$

where $h_2$ is 4 zero bits (corresponding to the padding applied by the Sinsemilla
[$\mathsf{pad}$ function](https://zips.z.cash/protocol/protocol.pdf#concretesinsemillahash)).

Each message piece is constrained by $\SinsemillaHash$ to its stated length. Additionally:
- $\DiversifiedTransmitBase$ and $\DiversifiedTransmitPublic$ are witnessed and checked
  to be valid elliptic curve points.
- $\mathsf{v}$ is witnessed as a field element, but its decomposition is sufficient to
  constrain it to be a 64-bit value.
- $\rho$ and $\psi$ are witnessed as field elements, so we know they are canonical.

However, we need additional constraints to enforce that:

- The chunks are the correct bit lengths (or else they could overlap in the decompositions
  and allow the prover to witness an arbitrary $\SinsemillaCommit$ message).
- The chunks contain the canonical decompositions of $\DiversifiedTransmitBase$,
  $\DiversifiedTransmitPublic$, $\rho$, and $\psi$ (or else the prover could witness
  multiple equivalent inputs to $\SinsemillaCommit$).

Some of these constraints are implemented with reusable circuit gadgets. We define a
custom gate controlled by a pair of selectors $(q_{\NoteCommit,1}, q_{\NoteCommit,2})$ to
hold the remaining constraints. We will need to witness 40 separate variables in a single
region, so we use two selectors that we activate on adjacent rows, in order to limit the
required rotations to the set `[Rotation::prev(), Rotation::cur(), Rotation::next()]`.

## Bit length constraints

Chunks $a$, $c$, and $f$ are directly constrained by Sinsemilla. For the remaining chunks,
we use the following constraints:

$$
\begin{array}{|c|l|}
\hline
\text{Degree} & \text{Constraint} \\\hline
  & \ShortLookupRangeCheck{b_0, 4} \\\hline
  & \ShortLookupRangeCheck{b_3, 4} \\\hline
  & \ShortLookupRangeCheck{d_2, 8} \\\hline
  & \ShortLookupRangeCheck{e_0, 6} \\\hline
  & \ShortLookupRangeCheck{e_1, 4} \\\hline
  & \ShortLookupRangeCheck{g_1, 9} \\\hline
  & \ShortLookupRangeCheck{h_0, 5} \\\hline
3 & q_{\NoteCommit,1} \cdot \BoolCheck{b_1} = 0 \\\hline
3 & q_{\NoteCommit,1} \cdot \BoolCheck{b_2} = 0 \\\hline
3 & q_{\NoteCommit,1} \cdot \BoolCheck{d_0} = 0 \\\hline
3 & q_{\NoteCommit,1} \cdot \BoolCheck{d_1} = 0 \\\hline
3 & q_{\NoteCommit,1} \cdot \BoolCheck{g_0} = 0 \\\hline
3 & q_{\NoteCommit,1} \cdot \BoolCheck{h_1} = 0 \\\hline
  & d_3 := z_{d,1} \\\hline
  & g_2 := z_{g,1} \\\hline
\end{array}
$$

where:
- $\BoolCheck{x} = x \cdot (1 - x)$.
- $\ShortLookupRangeCheck{}$ is a [short lookup range check](../decomposition.md#short-range-check).
- $z_{d,1}$ is the index-1 running sum output of $\SinsemillaHash(d),$ constrained by the
  hash to be 50 bits.
- $z_{g,1}$ is the index-1 running sum output of $\SinsemillaHash(g),$ constrained by the
  hash to be 240 bits.
- $d_3$ and $g_2$ are equality-constrained to their respective running sum outputs.

## Decomposition constraints

We have now derived or witnessed every subpiece, and range-constrained every subpiece:
- $a$ ($250$ bits) is witnessed and constrained outside the gate;
- $b_0$ ($4$ bits) is witnessed and constrained outside the gate;
- $b_1$ ($1$ bit) is witnessed and boolean-constrained in the gate;
- $b_2$ ($1$ bit) is witnessed and boolean-constrained in the gate;
- $b_3$ ($4$ bits) is witnessed and constrained outside the gate;
- $c$ ($250$ bits) is witnessed and constrained outside the gate;
- $d_0$ ($1$ bit) is witnessed and boolean-constrained in the gate;
- $d_1$ ($1$ bit) is witnessed and boolean-constrained in the gate;
- $d_2$ ($8$ bits) is witnessed and constrained outside the gate;
- $d_3$ ($50$ bits), equal to $z_{d,1}$;
- $e_0$ ($6$ bits) is witnessed and constrained outside the gate;
- $e_1$ ($4$ bit) is witnessed and constrained outside the gate;
- $f$ ($250$ bits) is witnessed and constrained outside the gate;
- $g_0$ ($1$ bit) is witnessed and boolean-constrained in the gate;
- $g_1$ ($9$ bits) is witnessed and constrained outside the gate;
- $g_2$ ($240$ bits), equal to $z_{g,1}$;
- $h_0$ ($5$ bits) is witnessed and constrained outside the gate;
- $h_1$ ($1$ bit) is witnessed and boolean-constrained in the gate;
- $h_2$ ($4$ bit) is a zero term, and can be omitted as the other chunks will not overlap it.

We can now use them to reconstruct both the (chunked) message pieces, and the original
field element inputs:

$$
\begin{align}
b &= b_0 + 2^4 \cdot b_1 + 2^5 \cdot b_2 + 2^6 \cdot b_3 \\
d &= d_0 + 2 \cdot d_1 + 2^2 \cdot d_2 + 2^{10} \cdot d_3 \\
e &= e_0 + 2^6 \cdot e_1 \\
g &= g_0 + 2 \cdot g_1 + 2^{10} \cdot g_2 \\
h &= h_0 + 2^5 \cdot h_1 \\
\mathsf{x(g_d)} &= a + 2^{250} \cdot b_0 + 2^{254} \cdot b_1 \\
\mathsf{x(pk_d)} &= b_3 + 2^4 \cdot c + 2^{254} \cdot d_0 \\
\mathsf{v} &= d_2 + 2^8 \cdot d_3 + 2^{58} \cdot e_0 \\
\rho &= e_1 + 2^4 \cdot f + 2^{254} \cdot g_0 \\
\psi &= g_1 + 2^9 \cdot g_2 + 2^{249} \cdot h_0 + 2^{254} \cdot h_1 \\
\end{align}
$$

$$
\begin{array}{|c|l|}
\hline
\text{Degree} & \text{Constraint} \\\hline
2 & q_{\NoteCommit,1} \cdot (b - (b_0 + b_1 \cdot 2^4 + b_2 \cdot 2^5 + b_3 \cdot 2^6)) = 0 \\\hline
2 & q_{\NoteCommit,1} \cdot (d - (d_0 + d_1 \cdot 2 + d_2 \cdot 2^2 + d_3 \cdot 2^{10})) = 0 \\\hline
2 & q_{\NoteCommit,1} \cdot (e - (e_0 + e_1 \cdot 2^6)) = 0 \\\hline
2 & q_{\NoteCommit,1} \cdot (g - (g_0 + g_1 \cdot 2 + g_2 \cdot 2^{10})) = 0 \\\hline
2 & q_{\NoteCommit,1} \cdot (h - (h_0 + h_1 \cdot 2^5)) = 0 \\\hline
2 & q_{\NoteCommit,1} \cdot (a + b_0 \cdot 2^{250} + b_1 \cdot 2^{254} - \mathsf{x(g_d)}) = 0 \\\hline
2 & q_{\NoteCommit,1} \cdot (b_3 + c \cdot 2^4 + d_0 \cdot 2^{254} - \mathsf{x(pk_d)} = 0 \\\hline
2 & q_{\NoteCommit,1} \cdot (d_2 + d_3 \cdot 2^8 + e_0 \cdot 2^{58} - \mathsf{v}) = 0 \\\hline
2 & q_{\NoteCommit,2} \cdot (e_1 + f \cdot 2^4 + g_0 \cdot 2^{254} - \rho) = 0 \\\hline
2 & q_{\NoteCommit,2} \cdot (g_1 + g_2 \cdot 2^9 + h_0 \cdot 2^{249} + h_1 \cdot 2^{254} - \psi) = 0 \\\hline
\end{array}
$$


Note that only the $ỹ$ LSB of the $y$-coordinates $\mathsf{y(g_d), y(pk_d)}$ was input to the hash, while the other bits of the $y$-coordinate were unused. However, we must still check that the witnessed $ỹ$ bit matches the original point's $y$-coordinate. The checks for $\mathsf{y(g_d), y(pk_d)}$ will follow the same format. For each $y$-coordinate, we witness:

$$
\begin{align}
y &= \textsf{LSB} \bconcat k_0 \bconcat k_1 \bconcat k_2 \bconcat k_3\\
  &= \textsf{LSB}
      \bconcat \text{ (bits $1..=9$ of $y$) }
      \bconcat \text{ (bits $10..=249$ of $y$) }
      \bconcat \text{ (bits $250..=253$ of $y$) }
      \bconcat \text{ (bit $254$ of $y$) },
\end{align}
$$

where $\textsf{LSB}$ is $b_2$ for $\mathsf{y(g_d)}$, and $d_1$ for $\mathsf{y(pk_d)}$. Let $$j = \textsf{LSB} + 2 \cdot k_0 + 2^{10} \cdot k_1.$$ We decompose $j$ to be $250$ bits using a strict $25-$word [ten-bit lookup](../decomposition.md#lookup-decomposition). The running sum outputs allow us to susbstitute $k_1 = z_{j, 1}.$

Recall that $b_2 = ỹ(g_d)$ and $d_1 = ỹ(pk_d)$ were pieces input to the Sinsemilla hash and have already been boolean-constrained. To constrain the remaining chunks, we use the following constraints:

$$
\begin{array}{|c|l|}
\hline
\text{Degree} & \text{Constraint} \\\hline
  & \ShortLookupRangeCheck{k_0, 9} \\\hline
  & \ShortLookupRangeCheck{k_2, 4} \\\hline
3 & q_{\NoteCommit,3} \cdot \BoolCheck{k_3} = 0 \\\hline
\end{array}
$$

Then, to check that the decomposition was correct:
$$
\begin{array}{|c|l|}
\hline
\text{Degree} & \text{Constraint} \\\hline
2 & q_{\NoteCommit,3} \cdot \left(j - (\textsf{LSB} + k_0 \cdot 2 + k_1 \cdot 2^{10}) \right) = 0 \\\hline
2 & q_{\NoteCommit,3} \cdot \left(y - (j + k_2 \cdot 2^{250} + k_3 \cdot 2^{254}) \right) = 0 \\\hline
\end{array}
$$

## Canonicity checks

At this point, we have constrained $\ItoLEBSP{\BaseLength{Orchard}}(\mathsf{x(g_d)})$,
$\ItoLEBSP{\BaseLength{Orchard}}(\mathsf{x(pk_d)})$,
$\ItoLEBSP{\BaseLength{Orchard}}(\rho)$, and $\ItoLEBSP{\BaseLength{Orchard}}(\psi)$ to be
255-bit values, with top bits $b_1$, $d_0$, $g_0$, and $h_1$ respectively. We have also
constrained:

$$
\begin{align}
\ItoLEBSP{\BaseLength{Orchard}}(\mathsf{x(g_d)}) &= \mathsf{x(g_d)} \pmod{q_\mathbb{P}} \\
\ItoLEBSP{\BaseLength{Orchard}}(\mathsf{x(pk_d)}) &= \mathsf{x(pk_d)} \pmod{q_\mathbb{P}} \\
\ItoLEBSP{\BaseLength{Orchard}}(\rho) &= \rho \pmod{q_\mathbb{P}} \\
\ItoLEBSP{\BaseLength{Orchard}}(\psi) &= \psi \pmod{q_\mathbb{P}} \\
\end{align}
$$

where $q_\mathbb{P}$ is the Pallas base field modulus. The remaining constraints will
enforce that these are indeed canonically-encoded field elements, i.e.

$$
\begin{align}
\ItoLEBSP{\BaseLength{Orchard}}(\mathsf{x(g_d)}) &< q_\mathbb{P} \\
\ItoLEBSP{\BaseLength{Orchard}}(\mathsf{x(pk_d)}) &< q_\mathbb{P} \\
\ItoLEBSP{\BaseLength{Orchard}}(\rho) &< q_\mathbb{P} \\
\ItoLEBSP{\BaseLength{Orchard}}(\psi) &< q_\mathbb{P} \\
\end{align}
$$

The Pallas base field modulus has the form $q_\mathbb{P} = 2^{254} + t_\mathbb{P}$, where
$$t_\mathbb{P} = \mathtt{0x224698fc094cf91b992d30ed00000001}$$
is 126 bits. We therefore know that if the top bit is not set, then the remaining bits
will always comprise a canonical encoding of a field element. Thus the canonicity checks
below are enforced if and only if the corresponding top bit is set to 1.

> In the constraints below we use a base-$2^{10}$ variant of the method used in libsnark
> (originally from [[SVPBABW2012](https://eprint.iacr.org/2012/598.pdf), Appendix C.1]) for
> range constraints $0 \leq x < t$:
>
> - Let $t'$ be the smallest power of $2^{10}$ greater than $t$.
> - Enforce $0 \leq x < t'$.
> - Let $x' = x + t' - t$.
> - Enforce $0 \leq x' < t'$.

### $\mathsf{x(g_d)}$ with $b_1 = 1 \implies \mathsf{x(g_d)} \geq 2^{254}$

In these cases, we check that $\mathsf{x(g_d)}_{0..=253} < t_\mathbb{P}$:

1. $b_1 = 1 \implies b_0 = 0.$

   Since $b_1 = 1 \implies \mathsf{x(g_d)}_{0..=253} < t_\mathbb{P} < 2^{126},$ we know that
   $\mathsf{x(g_d)}_{126..=253} = 0,$ and in particular
   $$b_0 := \mathsf{x(g_d)}_{250..=253} = 0.$$

2. $b_1 = 1 \implies 0 \leq a < t_\mathbb{P}.$

   To check that $a < t_\mathbb{P}$, we use two constraints:

    a) $0 \leq a < 2^{130}$. This is expressed in the custom gate as
       $$b_1 \cdot z_{a,13} = 0,$$
       where $z_{a,13}$ is the index-13 running sum output by $\SinsemillaHash(a).$

    b) $0 \leq a + 2^{130} - t_\mathbb{P} < 2^{130}$. To check this, we decompose
       $a' = a + 2^{130} - t_\mathbb{P}$ into thirteen 10-bit words (little-endian) using
       a running sum $z_{a'}$, looking up each word in a $10$-bit lookup table. We then
       enforce in the custom gate that
       $$b_1 \cdot z_{a',13} = 0.$$

$$
\begin{array}{|c|l|}
\hline
\text{Degree} & \text{Constraint} \\\hline
3 & q_{\NoteCommit,2} \cdot b_1 \cdot b_0 = 0 \\\hline
3 & q_{\NoteCommit,2} \cdot b_1 \cdot z_{a,13} = 0 \\\hline
2 & q_{\NoteCommit,1} \cdot (a + 2^{130} - t_\mathbb{P} - a') = 0 \\\hline
3 & q_{\NoteCommit,2} \cdot b_1 \cdot z_{a',13} = 0 \\\hline
\end{array}
$$

### $\mathsf{y(g_d)}$ with $k_3 = 1 \implies \mathsf{y(g_d)} \geq 2^{254}$

In these cases, we check that $\mathsf{y(g_d)}_{0..=253} < t_\mathbb{P}$:

1. $k_3 = 1 \implies k_2 = 0.$

   Since $k_3 = 1 \implies \mathsf{y(g_d)}_{0..=253} < t_\mathbb{P} < 2^{126},$ we know that
   $\mathsf{y(g_d)}_{126..=253} = 0,$ and in particular
   $$k_2 := \mathsf{y(g_d)}_{250..=253} = 0.$$

2. $k_3 = 1 \implies 0 \leq j < t_\mathbb{P}.$

   To check that $j < t_\mathbb{P}$, we use two constraints:

    a) $0 \leq j < 2^{130}$. This is expressed in the custom gate as
       $$k_3 \cdot z_{j,13} = 0,$$
       where $z_{j,13}$ is the index-13 running sum output by the $10$-bit lookup decomposition of $j$.

    b) $0 \leq j + 2^{130} - t_\mathbb{P} < 2^{130}$. To check this, we decompose
       $j' = j + 2^{130} - t_\mathbb{P}$ into thirteen 10-bit words (little-endian) using
       a running sum $z_{j'}$, looking up each word in a $10$-bit lookup table. We then
       enforce in the custom gate that
       $$k_3 \cdot z_{j',13} = 0.$$

$$
\begin{array}{|c|l|}
\hline
\text{Degree} & \text{Constraint} \\\hline
3 & q_{\NoteCommit,3} \cdot k_3 \cdot k_2 = 0 \\\hline
3 & q_{\NoteCommit,3} \cdot k_3 \cdot z_{j,13} = 0 \\\hline
2 & q_{\NoteCommit,3} \cdot (j + 2^{130} - t_\mathbb{P} - j') = 0 \\\hline
3 & q_{\NoteCommit,3} \cdot k_3 \cdot z_{j',13} = 0 \\\hline
\end{array}
$$

### $\mathsf{x(pk_d)}$ with $d_0 = 1 \implies \mathsf{x(pk_d)} \geq 2^{254}$

In these cases, we check that $\mathsf{x(pk_d)}_{0..=253} < t_\mathbb{P}$:

1. $d_0 = 1 \implies 0 \leq b_3 + 2^{4} \cdot c < t_\mathbb{P}.$

   To check that $0 \leq b_3 + 2^{4} \cdot c < t_\mathbb{P},$ we use two constraints:

    a) $0 \leq b_3 + 2^{4} \cdot c < 2^{140}.$ $b_3$ is already constrained individually
       to be a $4$-bit value. $z_{c,13}$ is the index-13 running sum output by
       $\SinsemillaHash(c).$ By constraining $$d_0 \cdot z_{c,13} = 0,$$ we constrain
       $b_3 + 2^4 \cdot c < 2^{134} < 2^{140}.$

    b) $0 \leq b_3 + 2^{4} \cdot c + 2^{140} - t_\mathbb{P} < 2^{140}$. To check this, we
       decompose ${b_3}c' = b_3 + 2^{4} \cdot c + 2^{140} - t_\mathbb{P}$ into fourteen
       10-bit words (little-endian) using a running sum $z_{{b_3}c'}$, looking up each
       word in a $10$-bit lookup table. We then enforce in the custom gate that
       $$d_0 \cdot z_{{b_3}c',14} = 0.$$

$$
\begin{array}{|c|l|}
\hline
\text{Degree} & \text{Constraint} \\\hline
3 & q_{\NoteCommit,2} \cdot d_0 \cdot z_{c,13} = 0 \\\hline
2 & q_{\NoteCommit,1} \cdot (b_3 + c \cdot 2^4 + 2^{140} - t_\mathbb{P} - {b_3}c') = 0 \\\hline
3 & q_{\NoteCommit,2} \cdot d_0 \cdot z_{{b_3}c',14} = 0 \\\hline
\end{array}
$$

### $\mathsf{y(pk_d)}$
This can be checked in exactly the same way as $\mathsf{y(g_d)}$, with $b_2$ replaced by $d_1$.

### $\rho$ with $g_0 = 1 \implies \rho \geq 2^{254}$

In these cases, we check that $\rho_{0..=253} < t_\mathbb{P}$:

1. $g_0 = 1 \implies 0 \leq e_1 + 2^{4} \cdot f < t_\mathbb{P}.$

   To check that $0 \leq e_1 + 2^{4} \cdot f < t_\mathbb{P},$ we use two constraints:

    a) $0 \leq e_1 + 2^{4} \cdot f < 2^{140}.$ $e_1$ is already constrained individually
       to be a $4$-bit value. $z_{f,13}$ is the index-13 running sum output by
       $\SinsemillaHash(f).$ By constraining $$g_0 \cdot z_{f,13} = 0,$$ we constrain
       $e_1 + 2^4 \cdot f < 2^{134} < 2^{140}.$

    b) $0 \leq e_1 + 2^{4} \cdot f + 2^{140} - t_\mathbb{P} < 2^{140}$. To check this, we
       decompose ${e_1}f' = e_1 + 2^{4} \cdot f + 2^{140} - t_\mathbb{P}$ into fourteen
       10-bit words (little-endian) using a running sum $z_{{e_1}f'}$, looking up each
       word in a $10$-bit lookup table. We then enforce in the custom gate that
       $$g_0 \cdot z_{{e_1}f',14} = 0.$$

$$
\begin{array}{|c|l|}
\hline
\text{Degree} & \text{Constraint} \\\hline
3 & q_{\NoteCommit,2} \cdot g_0 \cdot z_{f,13} = 0 \\\hline
2 & q_{\NoteCommit,1} \cdot (e_1 + f \cdot 2^4 + 2^{140} - t_\mathbb{P} - {e_1}f') = 0 \\\hline
3 & q_{\NoteCommit,2} \cdot g_0 \cdot z_{{e_1}f',14} = 0 \\\hline
\end{array}
$$

### $\psi$ with $h_1 = 1 \implies \psi \geq 2^{254}$

In these cases, we check that $\psi_{0..=253} < t_\mathbb{P}$:

1. $h_1 = 1 \implies h_0 = 0.$

   Since $h_1 = 1 \implies \psi_{0..=253} < t_\mathbb{P} < 2^{126},$ we know that $\psi_{126..=253} = 0,$
   and in particular $h_0 := \psi_{249..=253} = 0.$

2. $h_1 = 1 \implies 0 \leq g_1 + 2^{9} \cdot g_2 < t_\mathbb{P}.$

   To check that $0 \leq g_1 + 2^{9} \cdot g_2 < t_\mathbb{P},$ we use two constraints:

    a) $0 \leq g_1 + 2^{9} \cdot g_2 < 2^{140}.$ $g_1$ is already constrained individually
       to be a $9$-bit value. $z_{g,13}$ is the index-13 running sum output by
       $\SinsemillaHash(g).$ By constraining $$h_1 \cdot z_{g,13} = 0,$$ we constrain
       $g_1 + 2^9 \cdot g_2 < 2^{129} < 2^{130}.$

    b) $0 \leq g_1 + 2^{9} \cdot g_2 + 2^{130} - t_\mathbb{P} < 2^{130}$. To check this,
       we decompose ${g_1}{g_2}' = g_1 + 2^{9} \cdot g_2 + 2^{130} - t_\mathbb{P}$ into
       thirteen 10-bit words (little-endian) using a running sum $z_{{g_1}{g_2}'}$,
       looking up each word in a $10$-bit lookup table. We then enforce in the custom gate
       that $$h_1 \cdot z_{{g_1}{g_2}',13} = 0.$$

$$
\begin{array}{|c|l|}
\hline
\text{Degree} & \text{Constraint} \\\hline
3 & q_{\NoteCommit,2} \cdot h_1 \cdot h_0 = 0 \\\hline
3 & q_{\NoteCommit,2} \cdot h_1 \cdot z_{g,13} = 0 \\\hline
2 & q_{\NoteCommit,1} \cdot (g_1 + g_2 \cdot 2^9 + 2^{130} - t_\mathbb{P} - {g_1}{g_2}') = 0 \\\hline
3 & q_{\NoteCommit,2} \cdot g_0 \cdot z_{{g_1}{g_2}',13} = 0 \\\hline
\end{array}
$$

## Region layout

The constraints controlled by the $(q_{\NoteCommit,1}, q_{\NoteCommit,2})$ selectors are
arranged in a single region across 10 advice columns, requiring four rows.

$$
\begin{array}{|c|c|c|c|c|c|c|c|c|c|cc}
          &                &                &        &          &                   &          &                &          &           & q_{\NoteCommit,1} & q_{\NoteCommit,2} \\\hline
    b     &        d       &        e       &   g    &     h    &        d_1        &  x(pk_d) &      b_3       &     a'   &   b_2     &        0          &         0         \\\hline
  {e_1}f' &  {g_1}{g_2}'   &        v       &  d_2   &  z_{d,1} &        e_0        &{b_3}{c}' &       c        &     a    &  x(g_d)   &        1          &         0         \\\hline
   e_1    &        f       &       g_0      &  g_1   &  z_{g,1} &        h_0        &    h_1   &      d_0       &    b_0   &   b_1     &        0          &         1         \\\hline
  \rho    &    z_{f,13}    & z_{{e_1}f',14} &  \psi  & z_{g,13} &z_{{g_1}{g_2}',13} & z_{c,13} & z_{{b_3}c',14} & z_{a,13} & z_{a',13} &        0          &         0         \\\hline
\end{array}
$$
