package writes.filters

import org.scalatest.{FlatSpec, Matchers}
import util.ConnectorCapability.JoinRelationLinksCapability
import util._

class DeleteManyRelationFilterSpec extends FlatSpec with Matchers with ApiSpecBase {
  override def runOnlyForCapabilities = Set(JoinRelationLinksCapability)

  val schema =
    """model Top{
      |   id       String  @id @default(cuid())
      |   top      String
      |   bottomId String?
      |
      |   bottom Bottom? @relation(fields: [bottomId], references: [id])
      |}
      |
      |model Bottom{
      |   id           String  @id @default(cuid())
      |   bottom       String
      |   veryBottomId String?
      |
      |   top        Top?
      |   veryBottom VeryBottom? @relation(fields: [veryBottomId], references: [id])
      |}
      |
      |model VeryBottom{
      |   id         String @id @default(cuid())
      |   veryBottom String
      |   bottom     Bottom?
      |}""".stripMargin

  lazy val project: Project = SchemaDsl.fromStringV11() { schema }

  override protected def beforeAll(): Unit = {
    super.beforeAll()
    database.setup(project)
  }

  override def beforeEach(): Unit = database.truncateProjectTables(project)

  "The delete many Mutation" should "delete the items matching the where relation filter" in {
    createTop("top1")
    createTop("top2")

    server.query(
      s"""mutation {
         |  createTop(
         |    data: {
         |      top: "top3"
         |      bottom: {
         |        create: {bottom: "bottom1"}
         |      }
         |    }
         |  ) {
         |    id
         |  }
         |}
      """.stripMargin,
      project
    )

    val filter            = """{ bottom: { is: null }}"""
    val firstCount        = topCount
    val filterQueryCount  = server.query(s"""{tops(where: $filter){id}}""", project).pathAsSeq("data.tops").length
    val filterDeleteCount = server.query(s"""mutation {deleteManyTops(where: $filter){count}}""".stripMargin, project).pathAsLong("data.deleteManyTops.count")
    val lastCount         = topCount

    firstCount should be(3)
    filterQueryCount should be(2)
    firstCount - filterQueryCount should be(lastCount)
    firstCount - filterDeleteCount should be(lastCount)
  }

  "The delete many Mutation" should "delete all items if the filter is empty" in {
    createTop("top1")
    createTop("top2")

    server.query(
      s"""mutation {
         |  createTop(
         |    data: {
         |      top: "top3"
         |      bottom: {
         |        create: {bottom: "bottom1"}
         |      }
         |    }
         |  ) {
         |    id
         |  }
         |}
      """.stripMargin,
      project
    )

    val firstCount        = topCount
    val filterQueryCount  = server.query(s"""{tops{id}}""", project).pathAsSeq("data.tops").length
    val filterDeleteCount = server.query(s"""mutation {deleteManyTops{count}}""".stripMargin, project).pathAsLong("data.deleteManyTops.count")
    val lastCount         = topCount

    firstCount should be(3)
    filterQueryCount should be(3)
    firstCount - filterQueryCount should be(lastCount)
    firstCount - filterDeleteCount should be(lastCount)
  }

  "The delete many Mutation" should "work for deeply nested filters" in {
    createTop("top1")
    createTop("top2")

    server.query(
      s"""mutation {
         |  createTop(
         |    data: {
         |      top: "top3"
         |      bottom: {
         |        create: {
         |        bottom: "bottom1"
         |        veryBottom: {create: {veryBottom: "veryBottom"}}}
         |      }
         |    }
         |  ) {
         |    id
         |  }
         |}
      """.stripMargin,
      project
    )

    val filter            = """{ bottom: { is: { veryBottom: { is: { veryBottom: { equals: "veryBottom" }}}}}}"""
    val firstCount        = topCount
    val filterQueryCount  = server.query(s"""{tops(where: $filter){id}}""", project).pathAsSeq("data.tops").length
    val filterDeleteCount = server.query(s"""mutation {deleteManyTops(where: $filter){count}}""".stripMargin, project).pathAsLong("data.deleteManyTops.count")
    val lastCount         = topCount

    firstCount should be(3)
    filterQueryCount should be(1)
    firstCount - filterQueryCount should be(lastCount)
    firstCount - filterDeleteCount should be(lastCount)
  }

  "The delete many Mutation" should "work for named filters" in {
    createTop("top1")
    createTop("top2")

    server.query(
      s"""mutation {
         |  createTop(
         |    data: {
         |      top: "top3"
         |      bottom: {
         |        create: {
         |        bottom: "bottom1"
         |        veryBottom: { create: { veryBottom: "veryBottom" }}}
         |      }
         |    }
         |  ) {
         |    id
         |  }
         |}
      """.stripMargin,
      project
    )

    val filter            = """{ bottom: { is: { veryBottom: { is: { veryBottom: { equals: "veryBottom" }}}}}}"""
    val firstCount        = topCount
    val filterQueryCount  = server.query(s"""{tops(where: $filter){id}}""", project).pathAsSeq("data.tops").length
    val filterDeleteCount = server.query(s"""mutation {deleteManyTops(where: $filter){count}}""".stripMargin, project).pathAsLong("data.deleteManyTops.count")
    val lastCount         = topCount

    firstCount should be(3)
    filterQueryCount should be(1)
    firstCount - filterQueryCount should be(lastCount)
    firstCount - filterDeleteCount should be(lastCount)
  }

  def topCount: Int = server.query("{ tops { id } }", project).pathAsSeq("data.tops").size

  def createTop(top: String): Unit = {
    server.query(
      s"""mutation {
        |  createTop(
        |    data: {
        |      top: "$top"
        |    }
        |  ) {
        |    id
        |  }
        |}
      """.stripMargin,
      project
    )
  }
}
