use rust_web_ui::*;

pub struct Example32;

pub struct Example32Props {}

impl Default for Example32Props {
	fn default() -> Example32Props {
		Example32Props { }
	}
}

impl Widget<'_> for Example32 {
	type Props = Example32Props;

	fn render(mut ctx: Ctx<'_>, props: Example32Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Screen::Large(&[NoStyle::Noop("h-[985px]")]), Screen::Large(&[NoStyle::Noop("overflow-y-auto")])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("isolate"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("bg-white"), Style::PaddingX(24), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)]), Screen::Large(&[NoStyle::Noop("overflow-visible")]), Screen::Large(&[Style::PaddingX(0)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("-z-10"), NoStyle::Noop("overflow-hidden")]);
					
					props.child("1", Icon).run(|props| {
						props.style(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-[max(50%,25rem)]"), NoStyle::Noop("top-0"), NoStyle::Noop("h-[64rem]"), NoStyle::Noop("w-[128rem]"), NoStyle::Noop("-translate-x-1/2"), NoStyle::Noop("stroke-gray-200"), NoStyle::NoopGroup("[mask-image", NoStyle::Noop("radial-gradient(64rem_64rem_at_top,white,transparent)]"))]);
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("grid"), NoStyle::Noop("max-w-2xl"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-16"), Screen::Large(&[Style::MarginX(0)]), Screen::Large(&[NoStyle::Noop("max-w-none")]), Screen::Large(&[NoStyle::Noop("grid-cols-2")]), Screen::Large(&[NoStyle::Noop("items-start")]), Screen::Large(&[NoStyle::Noop("gap-y-10")])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[Screen::Large(&[NoStyle::Noop("col-span-2")]), Screen::Large(&[NoStyle::Noop("col-start-1")]), Screen::Large(&[NoStyle::Noop("row-start-1")]), Screen::Large(&[NoStyle::Noop("mx-auto")]), Screen::Large(&[NoStyle::Noop("grid")]), Screen::Large(&[NoStyle::Noop("w-full")]), Screen::Large(&[NoStyle::Noop("max-w-7xl")]), Screen::Large(&[NoStyle::Noop("grid-cols-2")]), Screen::Large(&[NoStyle::Noop("gap-x-8")]), Screen::Large(&[Style::PaddingX(32)])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Screen::Large(&[Style::PaddingRight(16)])]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Screen::Large(&[NoStyle::Noop("max-w-lg")])]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), NoStyle::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.set_text("Deploy faster"));
								});
								props.child("3", Dynamic).run("h1", |props| {
									props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-3xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-4xl")])]);
									
									props.child("0", Label).run(|props| props.set_text("A better workflow"));
								});
								props.child("5", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-xl"), NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(78))]);
									
									props.child("0", Label).run(|props| props.set_text("Aliquet nec orci mattis amet quisque ullamcorper neque, nibh sem. At arcu, sit dui mi, nibh dui, diam eget aliquam. Quisque id at vitae feugiat egestas."));
								});
							});
						});
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("-ml-12"), NoStyle::Noop("-mt-12"), Style::Padding(48), Screen::Large(&[NoStyle::Noop("sticky")]), Screen::Large(&[NoStyle::Noop("top-4")]), Screen::Large(&[NoStyle::Noop("col-start-2")]), Screen::Large(&[NoStyle::Noop("row-span-2")]), Screen::Large(&[NoStyle::Noop("row-start-1")]), Screen::Large(&[NoStyle::Noop("overflow-hidden")])]);
						
						props.child("1", Dynamic).run("img", |props| {
							props.styles(&[NoStyle::Noop("w-[48rem]"), NoStyle::Noop("max-w-none"), NoStyle::Noop("rounded-xl"), Style::Color(Color::Fg(100)), NoStyle::Noop("shadow-xl"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-gray-400/10"), Screen::Small(&[NoStyle::Noop("w-[57rem]")])]);
							props.set_attribute("src", "https://tailwindui.com/img/component-images/dark-project-app-screenshot.png");
							props.set_attribute("alt", "");
						});
					});
					props.child("5", Dynamic).run("div", |props| {
						props.styles(&[Screen::Large(&[NoStyle::Noop("col-span-2")]), Screen::Large(&[NoStyle::Noop("col-start-1")]), Screen::Large(&[NoStyle::Noop("row-start-2")]), Screen::Large(&[NoStyle::Noop("mx-auto")]), Screen::Large(&[NoStyle::Noop("grid")]), Screen::Large(&[NoStyle::Noop("w-full")]), Screen::Large(&[NoStyle::Noop("max-w-7xl")]), Screen::Large(&[NoStyle::Noop("grid-cols-2")]), Screen::Large(&[NoStyle::Noop("gap-x-8")]), Screen::Large(&[Style::PaddingX(32)])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Screen::Large(&[Style::PaddingRight(16)])]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("max-w-xl"), NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(78)), Screen::Large(&[NoStyle::Noop("max-w-lg")])]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.child("0", Label).run(|props| props.set_text("Faucibus commodo massa rhoncus, volutpat. Dignissim sed eget risus enim. Mattis mauris semper sed amet vitae sed turpis id. Id dolor praesent donec est. Odio penatibus risus viverra tellus varius sit neque erat velit. Faucibus commodo massa rhoncus, volutpat. Dignissim sed eget risus enim. Mattis mauris semper sed amet vitae sed turpis id."));
								});
								props.child("3", Dynamic).run("ul", |props| {
									props.set_attribute("role", "list");
									props.styles(&[Style::MarginTop(32), Style::SpaceY(32), Style::TextColor(Color::Fg(67))]);
									
									props.child("1", Dynamic).run("li", |props| {
										props.styles(&[Style::Flex, NoStyle::Noop("gap-x-3")]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[Style::MarginTop(4), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-none"), NoStyle::Noop("text-indigo-600")]);
										});
										props.child("3", Dynamic).run("span", |props| {
											props.child("0", Dynamic).run("strong", |props| {
												props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Push to deploy."));
											});
											props.child("1", Label).run(|props| props.set_text("Lorem ipsum, dolor sit amet consectetur adipisicing elit. Maiores impedit perferendis suscipit eaque, iste dolor cupiditate blanditiis ratione."));
										});
									});
									props.child("3", Dynamic).run("li", |props| {
										props.styles(&[Style::Flex, NoStyle::Noop("gap-x-3")]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[Style::MarginTop(4), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-none"), NoStyle::Noop("text-indigo-600")]);
										});
										props.child("3", Dynamic).run("span", |props| {
											props.child("0", Dynamic).run("strong", |props| {
												props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("SSL certificates."));
											});
											props.child("1", Label).run(|props| props.set_text("Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui lorem cupidatat commodo."));
										});
									});
									props.child("5", Dynamic).run("li", |props| {
										props.styles(&[Style::Flex, NoStyle::Noop("gap-x-3")]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[Style::MarginTop(4), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-none"), NoStyle::Noop("text-indigo-600")]);
										});
										props.child("3", Dynamic).run("span", |props| {
											props.child("0", Dynamic).run("strong", |props| {
												props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Database backups."));
											});
											props.child("1", Label).run(|props| props.set_text("Ac tincidunt sapien vehicula erat auctor pellentesque rhoncus. Et magna sit morbi lobortis."));
										});
									});
								});
								props.child("5", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(32)]);
									
									props.child("0", Label).run(|props| props.set_text("Et vitae blandit facilisi magna lacus commodo. Vitae sapien duis odio id et. Id blandit molestie auctor fermentum dignissim. Lacus diam tincidunt ac cursus in vel. Mauris varius vulputate et ultrices hac adipiscing egestas. Iaculis convallis ac tempor et ut. Ac lorem vel integer orci."));
								});
								props.child("7", Dynamic).run("h2", |props| {
									props.styles(&[Style::MarginTop(64), NoStyle::Noop("text-2xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("No server? No problem."));
								});
								props.child("9", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(24)]);
									
									props.child("0", Label).run(|props| props.set_text("Id orci tellus laoreet id ac. Dolor, aenean leo, ac etiam consequat in. Convallis arcu ipsum urna nibh. Pharetra, euismod vitae interdum mauris enim, consequat vulputate nibh. Maecenas pellentesque id sed tellus mauris, ultrices mauris. Tincidunt enim cursus ridiculus mi. Pellentesque nam sed nullam sed diam turpis ipsum eu a sed convallis diam."));
								});
							});
						});
					});
				});
			});
		});
	}
}
