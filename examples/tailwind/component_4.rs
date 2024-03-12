use rust_web_ui::*;

pub struct Example4;

pub struct Example4Props {}

impl Default for Example4Props {
	fn default() -> Example4Props {
		Example4Props { }
	}
}

impl Widget<'_> for Example4 {
	type Props = Example4Props;

	fn render(mut ctx: Ctx<'_>, props: Example4Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingY(48), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-4xl")]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::PaddingX(16), Screen::Small(&[Style::PaddingX(0)])]);
							
							props.child("1", Dynamic).run("h3", |props| {
								props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Applicant Information"));
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[Style::MarginTop(4), NoStyle::Noop("max-w-2xl"), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Personal details and application."));
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(24), NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-100")]);
							
							props.child("1", Dynamic).run("dl", |props| {
								props.styles(&[NoStyle::Noop("divide-y"), NoStyle::Noop("divide-gray-100")]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::PaddingX(16), Style::PaddingY(24), Screen::Small(&[NoStyle::Noop("grid")]), Screen::Small(&[NoStyle::Noop("grid-cols-3")]), Screen::Small(&[NoStyle::Noop("gap-4")]), Screen::Small(&[Style::PaddingX(0)])]);
									
									props.child("1", Dynamic).run("dt", |props| {
										props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("Full name"));
									});
									props.child("3", Dynamic).run("dd", |props| {
										props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(78)), Screen::Small(&[NoStyle::Noop("col-span-2")]), Screen::Small(&[Style::MarginTop(0)])]);
										
										props.child("0", Label).run(|props| props.set_text("Margot Foster"));
									});
								});
								props.child("3", Dynamic).run("div", |props| {
									props.styles(&[Style::PaddingX(16), Style::PaddingY(24), Screen::Small(&[NoStyle::Noop("grid")]), Screen::Small(&[NoStyle::Noop("grid-cols-3")]), Screen::Small(&[NoStyle::Noop("gap-4")]), Screen::Small(&[Style::PaddingX(0)])]);
									
									props.child("1", Dynamic).run("dt", |props| {
										props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("Application for"));
									});
									props.child("3", Dynamic).run("dd", |props| {
										props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(78)), Screen::Small(&[NoStyle::Noop("col-span-2")]), Screen::Small(&[Style::MarginTop(0)])]);
										
										props.child("0", Label).run(|props| props.set_text("Backend Developer"));
									});
								});
								props.child("5", Dynamic).run("div", |props| {
									props.styles(&[Style::PaddingX(16), Style::PaddingY(24), Screen::Small(&[NoStyle::Noop("grid")]), Screen::Small(&[NoStyle::Noop("grid-cols-3")]), Screen::Small(&[NoStyle::Noop("gap-4")]), Screen::Small(&[Style::PaddingX(0)])]);
									
									props.child("1", Dynamic).run("dt", |props| {
										props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("Email address"));
									});
									props.child("3", Dynamic).run("dd", |props| {
										props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(78)), Screen::Small(&[NoStyle::Noop("col-span-2")]), Screen::Small(&[Style::MarginTop(0)])]);
										
										props.child("0", Label).run(|props| props.set_text("margotfoster@example.com"));
									});
								});
								props.child("7", Dynamic).run("div", |props| {
									props.styles(&[Style::PaddingX(16), Style::PaddingY(24), Screen::Small(&[NoStyle::Noop("grid")]), Screen::Small(&[NoStyle::Noop("grid-cols-3")]), Screen::Small(&[NoStyle::Noop("gap-4")]), Screen::Small(&[Style::PaddingX(0)])]);
									
									props.child("1", Dynamic).run("dt", |props| {
										props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("Salary expectation"));
									});
									props.child("3", Dynamic).run("dd", |props| {
										props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(78)), Screen::Small(&[NoStyle::Noop("col-span-2")]), Screen::Small(&[Style::MarginTop(0)])]);
										
										props.child("0", Label).run(|props| props.set_text("$120,000"));
									});
								});
								props.child("9", Dynamic).run("div", |props| {
									props.styles(&[Style::PaddingX(16), Style::PaddingY(24), Screen::Small(&[NoStyle::Noop("grid")]), Screen::Small(&[NoStyle::Noop("grid-cols-3")]), Screen::Small(&[NoStyle::Noop("gap-4")]), Screen::Small(&[Style::PaddingX(0)])]);
									
									props.child("1", Dynamic).run("dt", |props| {
										props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("About"));
									});
									props.child("3", Dynamic).run("dd", |props| {
										props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(78)), Screen::Small(&[NoStyle::Noop("col-span-2")]), Screen::Small(&[Style::MarginTop(0)])]);
										
										props.child("0", Label).run(|props| props.set_text("Fugiat ipsum ipsum deserunt culpa aute sint do nostrud anim incididunt cillum culpa consequat. Excepteur qui ipsum aliquip consequat sint. Sit id mollit nulla mollit nostrud in ea officia proident. Irure nostrud pariatur mollit ad adipisicing reprehenderit deserunt qui eu."));
									});
								});
								props.child("11", Dynamic).run("div", |props| {
									props.styles(&[Style::PaddingX(16), Style::PaddingY(24), Screen::Small(&[NoStyle::Noop("grid")]), Screen::Small(&[NoStyle::Noop("grid-cols-3")]), Screen::Small(&[NoStyle::Noop("gap-4")]), Screen::Small(&[Style::PaddingX(0)])]);
									
									props.child("1", Dynamic).run("dt", |props| {
										props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("Attachments"));
									});
									props.child("3", Dynamic).run("dd", |props| {
										props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("col-span-2")]), Screen::Small(&[Style::MarginTop(0)])]);
										
										props.child("1", Dynamic).run("ul", |props| {
											props.set_attribute("role", "list");
											props.styles(&[NoStyle::Noop("divide-y"), NoStyle::Noop("divide-gray-100"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), NoStyle::Noop("border-gray-200")]);
											
											props.child("1", Dynamic).run("li", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween, Style::PaddingY(16), Style::PaddingLeft(16), Style::PaddingRight(20), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6")]);
												
												props.child("1", Dynamic).run("div", |props| {
													props.styles(&[Style::Flex, Style::Width(0), NoStyle::Noop("flex-1"), Style::ItemsCenter]);
													
													props.child("1", Icon).run(|props| {
														props.style(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
													});
													props.child("3", Dynamic).run("div", |props| {
														props.styles(&[Style::MarginLeft(16), Style::Flex, NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), NoStyle::Noop("gap-2")]);
														
														props.child("1", Dynamic).run("span", |props| {
															props.styles(&[NoStyle::Noop("truncate"), NoStyle::Noop("font-medium")]);
															
															props.child("0", Label).run(|props| props.set_text("resume_back_end_developer.pdf"));
														});
														props.child("3", Dynamic).run("span", |props| {
															props.styles(&[NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
															
															props.child("0", Label).run(|props| props.set_text("2.4mb"));
														});
													});
												});
												props.child("3", Dynamic).run("div", |props| {
													props.styles(&[Style::MarginLeft(16), NoStyle::Noop("flex-shrink-0")]);
													
													props.child("1", Dynamic).run("a", |props| {
														props.set_attribute("href", "#");
														props.styles(&[NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
														
														props.child("0", Label).run(|props| props.set_text("Download"));
													});
												});
											});
											props.child("3", Dynamic).run("li", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween, Style::PaddingY(16), Style::PaddingLeft(16), Style::PaddingRight(20), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6")]);
												
												props.child("1", Dynamic).run("div", |props| {
													props.styles(&[Style::Flex, Style::Width(0), NoStyle::Noop("flex-1"), Style::ItemsCenter]);
													
													props.child("1", Icon).run(|props| {
														props.style(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
													});
													props.child("3", Dynamic).run("div", |props| {
														props.styles(&[Style::MarginLeft(16), Style::Flex, NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), NoStyle::Noop("gap-2")]);
														
														props.child("1", Dynamic).run("span", |props| {
															props.styles(&[NoStyle::Noop("truncate"), NoStyle::Noop("font-medium")]);
															
															props.child("0", Label).run(|props| props.set_text("coverletter_back_end_developer.pdf"));
														});
														props.child("3", Dynamic).run("span", |props| {
															props.styles(&[NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
															
															props.child("0", Label).run(|props| props.set_text("4.5mb"));
														});
													});
												});
												props.child("3", Dynamic).run("div", |props| {
													props.styles(&[Style::MarginLeft(16), NoStyle::Noop("flex-shrink-0")]);
													
													props.child("1", Dynamic).run("a", |props| {
														props.set_attribute("href", "#");
														props.styles(&[NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
														
														props.child("0", Label).run(|props| props.set_text("Download"));
													});
												});
											});
										});
									});
								});
							});
						});
					});
				});
			});
		});
	}
}
